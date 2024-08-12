use std::path::PathBuf;

use crate::{core_components::snippet_manager::{SnippetManager, SnippetParameterBaseStorage, SnippetParameterComponent}, core_services::directory_manager::DirectoryManager, state_management::{external_snippet_manager::{self, ExternalSnippet, ExternalSnippetManager}, visual_snippet_component_manager::{self, VisualSnippetComponentManager}}, utils::sequential_id_generator::{self, SequentialIdGenerator, Uuid}};


// Initialized builder, containing all the information to build the snippets
pub struct InitializedPythonSnippetRunnerBuilder {
    build_information: Vec<PythonSnippetBuildInformation>,
    graph: petgraph::Graph<Uuid, (), petgraph::Directed>
    //graph
    //bi hash map of build information to graph node

}

pub struct PythonSnippetBuildInformation {
    visual_snippet_uuid: Uuid,
    parameters: Vec<SnippetParameterComponent>,
    python_file: PathBuf
}

impl Default for PythonSnippetBuildInformation {
    fn default() -> Self {
        return PythonSnippetBuildInformation {
            visual_snippet_uuid: Uuid::default(),
            parameters: Vec::default(),
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
}