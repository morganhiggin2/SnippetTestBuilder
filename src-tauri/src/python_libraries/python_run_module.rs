use std::{collections::{HashMap, HashSet, VecDeque}, env, fs::File, io::{self, Read}, path::PathBuf};

use petgraph::{graph::NodeIndex, visit::EdgeRef};
use pyo3::{types::{PyAnyMethods, PyDict, PyModule}, IntoPy, Py, PyAny, PyResult, Python};
use pathdiff::diff_paths;

use crate::{core_components::snippet_manager::{SnippetManager, SnippetParameterBaseStorage, SnippetParameterComponent}, core_services::{concurrent_processes::{get_runables_directory, get_working_directory}, directory_manager::DirectoryManager}, state_management::{external_snippet_manager::ExternalSnippetManager, visual_snippet_component_manager::VisualSnippetComponentManager}, utils::sequential_id_generator::{SequentialIdGenerator, Uuid}};


// location of the python runner library
const PYTHON_RUNNER_WRAPPER_LOCATION: &str = "snippet_runner.py";

// Initialized builder, containing all the information to build the snippets
pub struct InitializedPythonSnippetRunnerBuilder {
    // a map of each snippet id in the snippet manager to a snippet build information
    build_information: HashMap<Uuid, PythonSnippetBuildInformation>,
    graph: petgraph::stable_graph::StableGraph<Uuid, (), petgraph::Directed>,
    snippet_io_points_map: HashMap<(Uuid, String), Vec<(Uuid, String)>>
}

pub struct PythonSnippetBuildInformation {
    visual_snippet_uuid: Uuid,
    parameters: Vec<SnippetParameterComponent>,
    // names of inputs and outputs
    inputs: Vec<String>,
    outputs: Vec<String>,
    python_file: PathBuf
}

impl Default for PythonSnippetBuildInformation {
    fn default() -> Self {
        return PythonSnippetBuildInformation {
            visual_snippet_uuid: Uuid::default(),
            parameters: Vec::default(),
            inputs: Vec::<String>::default(),
            outputs: Vec::<String>::default(),
            python_file: PathBuf::default()
        }
    }
}

impl InitializedPythonSnippetRunnerBuilder {
    fn new (build_information: HashMap::<Uuid, PythonSnippetBuildInformation>, graph: petgraph::stable_graph::StableGraph<Uuid, (), petgraph::Directed>, snippet_io_points_map: HashMap<(Uuid, String), Vec<(Uuid, String)>> ) -> Self {
        return InitializedPythonSnippetRunnerBuilder {
            build_information: build_information,
            graph: graph,
            snippet_io_points_map: snippet_io_points_map
        };
    }

    /// Build the intialized python snippet runner
    pub fn build(snippet_manager: &SnippetManager, external_snippet_manager: &ExternalSnippetManager, directory_manager: &DirectoryManager, visual_snippet_component_manager: &VisualSnippetComponentManager,sequential_id_generator: &mut SequentialIdGenerator) -> Result<Self, String> {
        // create information necessary to 
        // a. run the necessary python code
        // b. call the front visual components

        // first make sure if it is even in a valid build state
        if !snippet_manager.validate_for_run() {
            return Err("Snippet project is not in a valid runstate, check for any inputs that are not assigned".to_string());
        }

        // build information
        let mut build_information = HashMap::<Uuid, PythonSnippetBuildInformation>::new();

        // get graph of connecting snippets, with each node weight being the snippet uuid
        let runtime_graph = snippet_manager.get_snippet_graph();

        // mapping of snippet inputs to outputs as dictated by pipelines 
        let snippet_io_points_map = snippet_manager.generate_snippet_io_point_mappings();

        // iterate over all the snippets
        for snippet in snippet_manager.get_snippets_as_ref() {
            // create empty snippet build information
            let mut python_snippet_build_information = PythonSnippetBuildInformation::default();

            // find snippet in visual snippet manager, get uuid
            python_snippet_build_information.visual_snippet_uuid = visual_snippet_component_manager.find_snippet_front_uuid(&snippet.get_uuid()).unwrap();

            // create deep copy and set parameter values
            python_snippet_build_information.parameters = snippet.get_parameters_as_copy();

            python_snippet_build_information.inputs = snippet.get_input_names();
            python_snippet_build_information.outputs = snippet.get_output_names();
           
            // get the runnable python file
            {
                // get external snippet uuid
                let external_snippet_id = snippet.get_external_snippet_id();

                // look up external snippet
                //TODO handle when external snippet is removed while snippet is still in project
                let external_snippet = external_snippet_manager.find_external_snippet(external_snippet_id).unwrap();
                let snippet_directory_entry = directory_manager.find_directory_entry(external_snippet.get_package_path()).unwrap();
                
                // get runnable python file path
                python_snippet_build_information.python_file = snippet_directory_entry.get_python_file()?;
            }

            // insert into build information
            build_information.insert(snippet.get_uuid(), python_snippet_build_information);
        }

        return Ok(Self::new(build_information, runtime_graph, snippet_io_points_map));
    }

    // run the python snippet runnere
    pub fn run(mut self) -> Result<(), String> {
        // inputs: reference to lock on the app handler
        
        //TODO every time we want to write a log, we aquire the lock and then release, rather than holding for build information
        //    BUT what else is going to want the lock? as are single threaded single project

        // set the pythonpath if not already set
        set_python_path();

        // aquire GI
        Python::with_gil(|py| -> Result<(), String> {
            // import python module for calling snippets (the wrapper function)
            let python_runner_wrapper_path: PathBuf = get_working_directory().join(get_runables_directory()).join(PYTHON_RUNNER_WRAPPER_LOCATION.to_string());
            
            let mut file = match File::open(python_runner_wrapper_path) {
                Ok(file) => file,
                Err(e) => {
                    return Err(format!("Could not open python runner wrapper: {}", e));
                }
            };

            // read the file 
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                io::Result::Ok(_) => (),
                io::Result::Err(e) => {
                    return Err(format!("Could not read the contents of the python runner wrapper file: {}", e)); 
                }
            };

            // import wrapper
            let python_wrapper = match PyModule::from_code_bound(
                py,
                &contents,
                "snippet_runner.py",
                "snippet_runner"
            ) {
                PyResult::Ok(some) => some,
                PyResult::Err(e) => {
                    return Err(format!("Could not create python runner wrapper code from main python file: {}", e.to_string()));
                }
            }; 

            let python_wrapper_run_snippet = match python_wrapper.getattr("run_snippet") {
                PyResult::Ok(some) => some,
                PyResult::Err(e) => {
                    return Err(format!("Could not get run snippet attribute from wrapper python file: {}", e.to_string()));
                }
            };

            // queue for BFS
            let mut run_queue = VecDeque::<NodeIndex>::new();
            // set of nodes which we already ran
            let mut run_set = HashSet::<NodeIndex>::new();

            /*
            Current mapping overview: 
            - graph: directed graph representing the flow of the build from one snippet to another snippet based on the io points
                the weights of each node are the corresponding snippet id
            - io mapping: maps 
                for each snippet id, maps it's input by input name (since names are unique based on io type (input or output) and snippet id)
                    to a list of entries, each entry being a) the name of the output in the output snippet it maps to and b) the output snippet id it maps too  
             */
            // contains the mapping of the next input, and the pyany value to be inserted
            let mut input_cache = HashMap::<(Uuid, String), Py<PyAny>>::new();

            // add all nodes which have no inputs 
            for node in self.graph.node_indices() {
                // if node has no edges in
                if self.graph.edges_directed(node, petgraph::Direction::Incoming).count() == 0 {
                    // if no neighbors
                    // add to run queue
                    run_queue.push_back(node);
                }
            }

            // while queue is not empty:
            loop {
                // pop item (which is next snippet to run) from queue
                let run_node = match run_queue.pop_front() {
                    Some(some) => some,
                    None => {
                        // if there is no run node, we are done; exit loop
                        break;
                    }
                };

                // if this had already been ran
                if run_set.contains(&run_node) {
                    // skip running this node again, continue
                    continue;
                }

                // get snippet id of the node to run
                // we can safely assume the node exists since we grabed it from the graph and the graph is not being modified
                let snippet_id = self.graph.node_weight(run_node).unwrap().to_owned();

                // get inputs for snippet
                // if this fails, there is a critical logic error in the code
                let snippet_python_build_information = self.build_information.remove(&snippet_id).unwrap();

                // grab input parameters from hash map
                // this maps each snippets output to the next snippet input id and name
                let mut output_mapping = HashMap::<String, (Uuid, String)>::new();

                //for each output
                for output in snippet_python_build_information.outputs {
                    match self.snippet_io_points_map.get(&(snippet_id, output.to_owned())) {
                        // if it exists in mapping, insert all into cache
                        Some(other_inputs) => {
                            for other_input in other_inputs {
                                // for each designated output
                                output_mapping.insert(output.to_owned(), other_input.to_owned());
                            }
                        }
                        // no mapping, then nothing to do
                        None => ()
                    }
                }     

                let mut input_mapping = HashMap::<String, Py<PyAny>>::new();

                // fetch inputs for input mapping
                for input in snippet_python_build_information.inputs {
                    // if there is an input supplied, then input it
                    // if there is no input, then do not include it
                    match input_cache.remove(&(snippet_id.to_owned(), input.to_owned())) {
                        Some(val) => {
                            // insert into input mapping
                            input_mapping.insert(input, val);
                        }
                        None => ()
                    };

                }
                

                let mut parameter_mapping = HashMap::<String, SnippetParameterBaseStorage>::new();

                for parameter in snippet_python_build_information.parameters {
                    parameter_mapping.insert(parameter.get_name(), parameter.get_storage().clone());
                } 

                // convert parameter mapping to python
                let py_parameter_mapping = parameter_mapping.into_py(py);

                // convert input mapping to python
                let py_input_mapping = input_mapping.into_py(py);

                // convert output mapping to python
                let py_output_mapping = output_mapping.into_py(py);


                // convert to python module 
                let py_path = file_path_to_py_path(snippet_python_build_information.python_file.to_owned())?;

                let kwargs = PyDict::new_bound(py);

                match kwargs.set_item("snippet_path", py_path.to_owned().into_py(py)) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(format!("Could not insert item into kwargs map: {}", e.to_string()));
                    }
                };
                match kwargs.set_item("function_inputs", py_input_mapping) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(format!("Could not insert item into kwargs map: {}", e.to_string()));
                    }
                };
                match kwargs.set_item("input_mappings", py_output_mapping) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(format!("Could not insert item into kwargs map: {}", e.to_string()));
                    }
                };
                match kwargs.set_item("parameter_values", py_parameter_mapping) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(format!("Could not insert item into kwargs map: {}", e.to_string()));
                    }
                };

                // execute pywrapper
                let run_result = match python_wrapper_run_snippet.call((), Some(&kwargs)) {
                    Ok(output) => output,
                    Err(e) => {
                        return Err(format!("Execution of snippet {} failed with error {}", py_path.to_owned(), e.to_string()));
                    },
                };

                // convert result into HashMap<(Uuid, String), Py<PyAny>>
                let output_results: HashMap::<(Uuid, String), Py<PyAny>> = match run_result.extract() {
                    Ok(some) => some, 
                    Err(e) => {
                        return Err(format!("Could not convert output type into HashMap::<(Uuid, String), Py<PyAny>> for snippet {}: {}", snippet_id, e.to_string()));
                    },
                }; 

                // for each output result
                for output_result in output_results.into_iter() {
                    // insert into input cache
                    input_cache.insert(output_result.0, output_result.1);
                }

                // inlude node in run set
                run_set.insert(run_node.clone());

                // get children of node, insert into run queue 
                for edge in self.graph.edges_directed(run_node, petgraph::Direction::Outgoing) {
                    let child_node = edge.target();

                    run_queue.push_back(child_node);
                }
            }


                    // if we are missing any parameters, then we are still waiting on a child to run, 
                    //   so reinsert into the queue and continue

                // call snippet with HashMap<output_name, i32 number of copies) 

                // get mapping entry of outputs to inputs, include this in the wrapper module
                // note that not every output is assigned an input, so we want to check for the number of copies we need of each

                // get return
                // parse return into HashMap<string, Vec<PyAny>> from PyAny
                // this is the output name, and the copies of pyany

                // use the mapping and reducing to insert into input cache accordingly

                // add child dependencies to queue


            return Ok(());
        })?;
        
        return Ok(());
    }
}

fn file_path_to_py_path(mut path: PathBuf) -> Result<String, String> {
    // remove file extension from end of path
    path.set_extension("");

    // get working directory
    let mut runables_directory = get_runables_directory();
    // pop runables directory from path to get base
    runables_directory.pop();

    // build base python runner location
    let base_python_runner_location = runables_directory.to_owned();
    //base_python_runner_location.push(PYTHON_BASE_RUNNER_LOCATION.to_owned());

    // remove relative directory of runner files
    let runnable_relative_snippet_path = match diff_paths(path.to_owned(), base_python_runner_location.to_owned()) {
        Some(some) => some,
        None => {
            return Err(format!("Could not compute relative paths for path {} and {} in python run module", base_python_runner_location.to_string_lossy(), path.to_owned().to_string_lossy()));
        }
    };

    let mut py_path = String::new();

    // build python path
    for component in runnable_relative_snippet_path.iter() {
        let component_str = component.to_str().unwrap();
        py_path.push_str(component_str);
        py_path.push('.');
    }

    // remove any .
    py_path.remove(py_path.len() - 1);

    return Ok(py_path);
}

/// Set the python path, if it has not already been set, to make the runables visible to the python interpreter
pub fn set_python_path() {
    // get runables location
    let mut runables_directory = get_runables_directory();

    // pop runables off to get snippet directory base 
    runables_directory.pop();
    let runables_directory_str = runables_directory.to_str().unwrap();

    // set python path environment variable
    match env::var("PYTHONPATH") {
        Ok(val) => {
            // if it does not already contains the runables path, set it
            if !val.contains(runables_directory_str) {
                let mut seperator = ":";

                // if windows, the seperator is different
                if cfg!(target_os = "windows") {
                    seperator = ";";
                }

                let new_val = val + seperator + runables_directory_str;

                unsafe {
                    env::set_var("PYTHONPATH", new_val);
                }
            }
            // else, leave alone
        }
        // None could be found or error retriving, assume none existed
        Err(_) => {
            unsafe {
                env::set_var("PYTHONPATH", runables_directory.to_str().unwrap());
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{core_services::concurrent_processes::{get_runables_directory, get_working_directory}, python_libraries::python_run_module::file_path_to_py_path};

    #[test]
    fn test_file_path_to_py_path() {
        // get working directory
        let working_directory = get_working_directory();
        // create path buf
        let path = working_directory.join(get_runables_directory().join(PathBuf::from("snippets/root/main/basic_one_snippet/app")));

        let py_path = file_path_to_py_path(path);

        // this is the path because the python interpreter is running from the location of the executable of the program, not the file it runs 
        assert_eq!(py_path, Ok("runables.snippets.root.main.basic_one_snippet.app".to_string()));
    }
}