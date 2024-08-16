use std::{collections::{HashMap, VecDeque}, env, fs::File, io::{self, Read}, path::PathBuf};

use petgraph::graph::NodeIndex;
use pyo3::{types::{PyAnyMethods, PyModule}, Bound, IntoPy, Py, PyAny, PyResult, Python};

use crate::{core_components::snippet_manager::{SnippetManager, SnippetParameterBaseStorage, SnippetParameterComponent}, core_services::directory_manager::DirectoryManager, state_management::{external_snippet_manager::{self, ExternalSnippet, ExternalSnippetManager}, visual_snippet_component_manager::{self, VisualSnippetComponentManager}}, utils::sequential_id_generator::{self, SequentialIdGenerator, Uuid}};

use super::python_build_module::FinalizedPythonSnipppetInitializerBuilder;

// location of the python runner library
const PYTHON_RUNNER_WRAPPER_LOCATION: &str = "../python_api/snippet_runner";

// Initialized builder, containing all the information to build the snippets
pub struct InitializedPythonSnippetRunnerBuilder {
    // a map of each snippet id in the snippet manager to a snippet build information
    build_information: HashMap<Uuid, PythonSnippetBuildInformation>,
    graph: petgraph::Graph<Uuid, (), petgraph::Directed>,
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
    fn new (build_information: HashMap::<Uuid, PythonSnippetBuildInformation>, graph: petgraph::Graph<Uuid, (), petgraph::Directed>, snippet_io_points_map: HashMap<(Uuid, String), Vec<(Uuid, String)>> ) -> Self {
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
           
            // get the runnable python file
            {
                // get external snippet uuid
                let external_snippet_id = snippet.get_external_snippet_id();

                // look up external snippet
                //TODO handle when external snippet is removed while snippet is still in project
                let external_snippet = external_snippet_manager.find_external_snippet(external_snippet_id).unwrap();

                //TODO get file location, which means finding the snippet directory entry from the directory manager
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

        // aquire GI
        Python::with_gil(|py| -> Result<(), String> {
            // import python module for calling snippets (the wrapper function)
            let python_runner_wrapper_path: PathBuf = PYTHON_RUNNER_WRAPPER_LOCATION.into();
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
                    //TODO return error
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

            // queue for BFS
            let mut run_queue = VecDeque::<NodeIndex>::new();

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

                // get snippet id of the node to run
                // we can safely assume the node exists since we grabed it from the graph and the graph is not being modified
                let snippet_id = self.graph.node_weight(run_node).unwrap().to_owned();

                // get inputs for snippet
                // if this fails, there is a critical logic error in the code
                let mut snippet_python_build_information = self.build_information.remove(&snippet_id).unwrap();

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
                    // can unwrap safely as this logic is enforced by the pipelines
                    let value = input_cache.remove(&(snippet_id.to_owned(), input.to_owned())).unwrap();

                    // insert into input mapping
                    input_mapping.insert(input, value);
                }

                let mut parameter_mapping = HashMap::<String, Py::<PyAny>>::new();

                // get parameters
                for parameter in snippet_python_build_information.parameters.into_iter() {
                    let parameter_storage = parameter.get_storage();

                    // convert into pytype
                    let parameter_value_to_py = parameter_storage.to_owned().into_py(py);

                    // insert into parameter mapping 
                    parameter_mapping.insert(parameter.get_name(), parameter_value_to_py);
                }

                // convert input mapping to python
                let py_input_mapping = input_mapping.into_py(py);

                // convert output mapping to python
                let py_output_mapping = output_mapping.into_py(py);

                // convert parameter mapping to python
                let py_parameter_mapping = parameter_mapping.into_py(py);

                // convert file path to python module 
                let py_path = file_path_to_py_path(snippet_python_build_information.python_file);


                let mut kwargs = HashMap::<&str, Py<PyAny>>::new();

                kwargs.insert("snippet_path", py_path.into_py(py));
                kwargs.insert("function_inputs", py_input_mapping);
                kwargs.insert("input_mappings", py_output_mapping);
                kwargs.insert("parameter_values", py_parameter_mapping);

                // execute pywrapper
                let returns = match python_wrapper.call((), Some(Bound::from_owned_ptr(py,&kwargs.into_py(py)))) {
                    Ok(output) => output,
                    Err(e) => {
                        return Err(format!("Execution of snippet {} failed with error {}", py_path.to_owned(), e.to_string()));
                    },
                };

                // reinsert snippet python build information
                self.build_information.insert(snippet_id, snippet_python_build_information);
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

fn file_path_to_py_path(path: PathBuf) -> String {
    // get working directory
    let working_directory = env::current_dir().expect("Failed to get current directory");

    // get path relative to working directory
    let relative_directory = path.strip_prefix(&working_directory).unwrap_or(&path);

    let mut py_path = String::new();

    // build python path
    for component in relative_directory.iter() {
        let component_str = component.to_str().unwrap();
        py_path.push_str(component_str);
        py_path.push('.');
    }

    // remove any .
    py_path.remove(py_path.len() - 1);

    return py_path;
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::python_libraries::python_run_module::{file_path_to_py_path, InitializedPythonSnippetRunnerBuilder};

    #[test]
    fn test_file_path_to_py_path() {
        // create path buf
        let path = PathBuf::from("one/two/three/four");

        let py_path = file_path_to_py_path(path);

        assert_eq!(py_path, "one.two.three.four");
    }
}