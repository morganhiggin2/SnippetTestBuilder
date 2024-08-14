use std::{collections::VecDeque, fs::File, io::{self, Read}, path::PathBuf};

use pyo3::{types::PyModule, PyResult, Python};

use crate::{core_components::snippet_manager::{SnippetManager, SnippetParameterBaseStorage, SnippetParameterComponent}, core_services::directory_manager::DirectoryManager, state_management::{external_snippet_manager::{self, ExternalSnippet, ExternalSnippetManager}, visual_snippet_component_manager::{self, VisualSnippetComponentManager}}, utils::sequential_id_generator::{self, SequentialIdGenerator, Uuid}};

use super::python_build_module::FinalizedPythonSnipppetInitializerBuilder;

// location of the python runner library
const PYTHON_RUNNER_WRAPPER_LOCATION: &str = "../python_api/snippet_runner";

// Initialized builder, containing all the information to build the snippets
pub struct InitializedPythonSnippetRunnerBuilder {
    build_information: Vec<PythonSnippetBuildInformation>,
    graph: petgraph::Graph<Uuid, (), petgraph::Directed>
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
    fn new (build_information: Vec::<PythonSnippetBuildInformation>, graph: petgraph::Graph<Uuid, (), petgraph::Directed>) -> Self {
        return InitializedPythonSnippetRunnerBuilder {
            build_information: build_information,
            graph: graph
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
        let build_information = Vec::<PythonSnippetBuildInformation>::new();

        // get graph of connecting snippets, with each node weight being the snippet uuid
        let runtime_graph = snippet_manager.get_snippet_graph();

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
        }

        return Ok(Self::new(build_information, runtime_graph));
    }

    // run the python snippet runnere
    pub fn run(self) -> Result<(), String> {
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
            let run_queue = VecDeque::<String>::new();

            /*
            Current mapping overview: 
            - graph: directed graph representing the flow of the build from one snippet to another snippet based on the io points
                the weights of each node are the corresponding snippet id
            - io mapping: maps 
                for each snippet id, maps it's input by input name (since names are unique based on io type (input or output) and snippet id)
                    to a list of entries, each entry being a) the name of the output in the output snippet it maps to and b) the output snippet id it maps too  
             */

            // TODO need a mapping of input snippets to the snippet io maps, where it maps the input name to the output name (snippet_id) -> [(input_snippet_name, output_snippet_name, output_snippet_id), ...]
            //   we then insert these into the hashmap, where the receiving snippet can query for them from the hashmap by snippet_id
            // when execution is complete, we need to get the next input snippet ids from the outputs 
            // ok, but once it completes, how do we know? well, we know because we a) have the snippet id in the node and b) the name

            // hashmap for outputs, key is , value is PyAny
            
            // add root node of graph to queue

            // while queue is not empty:
                // pop item (which is next snippet to run) from queue

                // grab input parameters from hash map

                    // if we are missing any parameters, then we are still waiting on a child to run, 
                    //   so reinsert into the queue and continue

                // call snippet with input parameters with wrapper python module

                // get returns

                // insert returns into hashmap for outputs

                // add child dependencies to queue


            return Ok(());
        })?;

        return Ok(());
    }
}
