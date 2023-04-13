use crate::{state_management::{ApplicationState, window_manager::WindowSession, external_snippet_manager::{ExternalSnippet, IOContentType}}, utils::sequential_id_generator::{SequentialIdGenerator, self}};
use std::{sync::MutexGuard, collections::HashMap};
use bimap::BiHashMap;
use tauri::window;
use crate::utils::sequential_id_generator::Uuid;

/// the manager of the snippets, and their links
pub struct SnippetManager {
    //list of uuid of snippets to index in edge adj list
    //edge adj list for snippets
    //list of components 
    snippets: HashMap<Uuid, SnippetComponent>,
    pipelines: HashMap<Uuid, PipelineComponent>,

    //mapping for pipeline connects to pipeline components
    to_pipeline_connector_to_pipeline: HashMap<Uuid, Uuid>,
    from_pipeline_connector_to_pipeline: HashMap<Uuid, Uuid>,
    snippet_to_pipeline_components: HashMap<Uuid, Uuid>

    //mapping for snippets and pipelines
    //list of uuid of snippets to index in edge adj list
    //uuid_to_edge_adj_index: HashMap<u32, usize>
    //edge adj list
}

/// the actual snippet itself
struct SnippetComponent {
    uuid: Uuid,
    external_snippet_uuid: Uuid,
    pipeline_connectors: Vec<PipelineConnectorComponent> 
}

pub struct PipelineConnectorComponent {
    uuid: Uuid,
    external_io_point_uuid: Uuid,
    name: String,
    content_type: IOContentType,
    input: bool
}

struct PipelineComponent {
    uuid: Uuid, 
    from_pipeline_connector_uuid: Uuid,
    to_pipeline_connector_uuid: Uuid
}

impl Default for SnippetManager {
    fn default() -> Self {
        return SnippetManager {
            snippets: HashMap::with_capacity(12),
            pipelines: HashMap::with_capacity(12),
            from_pipeline_connector_to_pipeline: HashMap::with_capacity(24),
            to_pipeline_connector_to_pipeline: HashMap::with_capacity(24),
            snippet_to_pipeline_components: HashMap::with_capacity(24)
            //uuid_to_edge_adj_index: HashMap::with_capacity(24),
        };
    }
}

impl SnippetManager {
    /// create a new snippet
    pub fn new_snippet(seq_id_generator: &mut SequentialIdGenerator, window_session: &mut WindowSession, external_snippet: &mut ExternalSnippet) -> Uuid {
        //create snippet component
        let mut snippet_component : SnippetComponent = SnippetComponent::new(seq_id_generator);

        //get snippet uuid before borrowed mut
        let snippet_uuid : Uuid = snippet_component.uuid;

        //add components from external snippet to snippet
        snippet_component.external_snippet_uuid = external_snippet.get_uuid();

        //add io points to snippet component as pipeline connectors
        let pipeline_connectors = external_snippet.get_io_points_as_pipeline_connectors(seq_id_generator);

        //add pipeline connector uuid to snippet mapping
        for pipeline_connector in pipeline_connectors.iter() {
            window_session.snippet_manager.snippet_to_pipeline_components.insert(pipeline_connector.uuid, snippet_uuid);
        }

        //move pipeline connectors to snippet component
        snippet_component.pipeline_connectors = pipeline_connectors;

        //add to snippets list in snippet manager
        window_session.snippet_manager.snippets.insert(snippet_uuid, snippet_component);

        //return uuid of snippet
        return snippet_uuid;
    }

    /// find pipeline from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline 
    fn find_pipeline(&mut self, uuid: &Uuid) -> Option<&mut PipelineComponent>{
        //find pipeline in vector
        return self.pipelines.get_mut(uuid);
    }
    
    /// find snippet from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the snippet 
    fn find_snippet(&mut self, uuid: &Uuid) -> Option<&mut SnippetComponent>{
        //find pipeline in vector
        return self.snippets.get_mut(uuid);
    }

    /// find pipeline connector uuid from pipeipe uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector going into pipeline
    pub fn find_pipeline_uuid_from_from_pipeline_connector(&mut self, uuid: &Uuid) -> Option<Uuid> {
        //find uuid of pipeline connector
        return self.from_pipeline_connector_to_pipeline.get(uuid).cloned(); 
    }
    
    /// find pipeline connector uuid from pipeipe uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector coming out of pipeline
    pub fn find_pipeline_uuid_from_to_pipeline_connector(&mut self, uuid: &Uuid) -> Option<Uuid> {
        //find uuid of pipeline connector
        return self.to_pipeline_connector_to_pipeline.get(uuid).cloned(); 
    }
    /// find snippet uuid from pipeline uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector 
    pub fn find_snippet_uuid_from_pipeline_connector(&mut self, uuid: &Uuid) -> Option<Uuid> {
        return self.snippet_to_pipeline_components.get(uuid).cloned();
    }

    /// create pipeline
    /// 
    /// # Arguments
    /// * 'from_uuid' from pipeline connector's uuid
    /// * 'to uuid' to pipeline connector's uuid
    pub fn create_pipeline(seq_id_generator: &mut SequentialIdGenerator, window_session: &mut WindowSession, from_uuid: Uuid, to_uuid: Uuid) -> Result<Uuid, &'static str> {
        //find pipeline connectors
        //find pipeline connector uuid
        let from_snippet_uuid = match window_session.snippet_manager.find_snippet_uuid_from_pipeline_connector(&from_uuid) {
            Some(result) => result,
            None => {
                return Err("from snippet uuid from pipeline connector not found");
            }
        };

        let from_snippet = match window_session.snippet_manager.find_snippet(&from_snippet_uuid) {
            Some(result) => result,
            None => {
                return Err("from snippet from pipeline connector not found")
            }
        };

        //verify pipeline connector exists in pipeline
        match from_snippet.find_pipeline_connector(from_uuid) {
            Some(_) => (),
            None => {
                return Err("from pipeline connector does not exist in snippet");
            }
        } 

        //find pipeline connector uuid
        let to_snippet_uuid = match window_session.snippet_manager.find_snippet_uuid_from_pipeline_connector(&to_uuid) {
            Some(result) => result,
            None => {
                return Err("to snippet uuid from pipeline connector not found");
            }
        };

        let to_snippet = match window_session.snippet_manager.find_snippet(&to_snippet_uuid) {
            Some(result) => result,
            None => {
                return Err("to snippet from pipeline connector not found")
            }
        };

        //verify pipeline connector exists in pipeline
        match to_snippet.find_pipeline_connector(from_uuid) {
            Some(_) => (),
            None => {
                return Err("to pipeline connector does not exist in snippet");
            }
        } 

        //create new pipeline
        let pipeline_component = PipelineComponent::new(seq_id_generator, &from_uuid, &to_uuid);

        //get pipeline uuid for return
        let pipeline_uuid = pipeline_component.get_uuid();

        //add pipeline connecting values to snippet manager
        window_session.snippet_manager.from_pipeline_connector_to_pipeline.insert(from_uuid, pipeline_uuid);
        window_session.snippet_manager.to_pipeline_connector_to_pipeline.insert(to_uuid, pipeline_uuid);

        //add to snippet manager
        window_session.snippet_manager.pipelines.insert(pipeline_uuid, pipeline_component);

        //return uuid of new pipeline
        return Ok(pipeline_uuid);
    }
}

impl SnippetComponent {
    pub fn new(seq_id_generator: &mut SequentialIdGenerator) -> Self {
        return SnippetComponent {
            uuid: seq_id_generator.get_id(),
            external_snippet_uuid: 0,
            pipeline_connectors: Vec::new()
        }
    }

    /// find pipeline connector from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector
    fn find_pipeline_connector(&mut self, uuid: Uuid) -> Option<&mut PipelineConnectorComponent>{
        //find pipeline in vector
        return self.pipeline_connectors.iter_mut().find(|pipe: &&mut PipelineConnectorComponent| pipe.uuid == uuid);
    }
}

impl PipelineConnectorComponent {
    pub fn new(seq_id_generator: &mut SequentialIdGenerator, external_io_point_uuid: Uuid, name: &str, content_type: &IOContentType, input: bool) -> Self {
        return PipelineConnectorComponent {
            uuid: seq_id_generator.get_id(),
            external_io_point_uuid: external_io_point_uuid,
            name: name.clone().to_string(),
            content_type: content_type.clone(),
            input: input
        }
    }
}

impl PipelineComponent {
    pub fn new(seq_id_generator: &mut SequentialIdGenerator, from_uuid: &Uuid, to_uuid: &Uuid) -> Self {
        return PipelineComponent {
            uuid: seq_id_generator.get_id(),
            from_pipeline_connector_uuid: from_uuid.clone(),
            to_pipeline_connector_uuid: to_uuid.clone()
        }
    }

    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }
}

//map: pipeline_connectors->parent

//could store parent uuid for connector inside connector or mapping