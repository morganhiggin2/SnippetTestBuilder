use crate::{core_components::snippet_manager::SnippetManager, state_management::{external_snippet_manager::{self, ExternalSnippet, ExternalSnippetManager}, visual_snippet_component_manager::{self, VisualSnippetComponentManager}}, utils::sequential_id_generator::{self, SequentialIdGenerator, Uuid}};


// Initialized builder, containing all the information to build the snippets
pub struct InitializedPythonSnippetRunnerBuilder {
    //build_information: Vec<PythonSnippetBuildInformation>
    //graph
    //bi hash map of build information to graph node
}

pub struct PythonSnippetBuildInformation {
    visual_snippet_uuid: Uuid
}

impl Default for PythonSnippetBuildInformation {
    fn default() -> Self {
        return PythonSnippetBuildInformation {
            visual_snippet_uuid: Uuid::default()
        }
    }
}

/*
// the build information for each snippet
pub struct PythonSnippetBuildInformation {
    snippet_uuid: snippet_uuid,
    // visual uuid too? for emits?
    name: String,
    path: PathBuf
}

// the state of the builder once the snippets have been built
pub struct FinalizedPythonSnipppetInitializerBuilder {
    
}

pub struct PythonSnippetBuilderWrapper {
    directory_entry_uuid: Uuid,
    python_snippet_builder: PythonSnippetBuilder,
}*/

impl InitializedPythonSnippetRunnerBuilder {
    pub fn build(snippet_manager: &SnippetManager, external_snippet_manager: &ExternalSnippetManager, visual_snippet_component_manager: &VisualSnippetComponentManager,sequential_id_generator: &mut SequentialIdGenerator) -> Result<(), String> {
        // create information necessary to 
        // a. run the necessary python code
        // b. call the front visual components

        // iterate over all the snippets
        for snippet in snippet_manager.get_snippets_as_ref() {
            // create empty snippet build information
            let mut python_snippet_build_information = PythonSnippetBuildInformation::default();
            
            // find snippet in visual snippet manager, get uuid
            python_snippet_build_information.visual_snippet_uuid = visual_snippet_component_manager.find_snippet_front_uuid(&snippet.get_uuid()).unwrap();

            // get external snippet uuid
            let external_snippet_id = snippet.get_external_snippet_id();

            // look up external snippet
            //TODO handle when external snippet is removed while snippet is still in project
            let external_snippet = external_snippet_manager.find_external_snippet(external_snippet_id).unwrap();

            //TODO get file location, which means finding the snippet directory entry from the directory manager

            // get graph of connecting snippets, with each node weight being the snippet uuid
            let runtime_graph = snippet_manager.get_snippet_graph();
        }

        return Ok(());
    }
}