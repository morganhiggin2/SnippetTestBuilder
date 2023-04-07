use crate::{state_management::{ApplicationState, window_manager::WindowSession, external_snippet_manager::{ExternalSnippet, IOContentType}}, utils::sequential_id_generator::{SequentialIdGenerator, self}};
use std::{sync::MutexGuard, collections::HashMap};
use bimap::BiHashMap;
use crate::utils::sequential_id_generator::Uuid;

/// the manager of the snippets, and their links
pub struct SnippetManager {
    //list of uuid of snippets to index in edge adj list
    //edge adj list for snippets
    //list of components 
    snippets: Vec<SnippetComponent>,
    pipelines: Vec<PipelineComponent>,

    //mapping for pipeline connects to pipeline components
    pipeline_connector_to_pipeline: HashMap<Uuid, Uuid>

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
}

impl Default for SnippetManager {
    fn default() -> Self {
        return SnippetManager {
            snippets: Vec::with_capacity(12),
            pipelines: Vec::with_capacity(12),
            pipeline_connector_to_pipeline: HashMap::with_capacity(24)
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
        snippet_component.pipeline_connectors = pipeline_connectors;

        //add to snippets list in snippet manager
        window_session.snippet_manager.snippets.push(snippet_component);

        //return uuid of snippet
        return snippet_uuid;
    }

    /// find pipeline from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline 
    fn find_pipeline(&mut self, uuid: Uuid) -> Option<&mut PipelineComponent>{
        //find pipeline in vector
        return self.pipelines.iter_mut().find(|pipe: &&mut PipelineComponent | pipe.uuid == uuid);
    }
    
    /// find snippet from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the snippet 
    fn find_snippet(&mut self, uuid: Uuid) -> Option<&mut SnippetComponent>{
        //find pipeline in vector
        return self.snippets.iter_mut().find(|pipe: &&mut SnippetComponent| pipe.uuid == uuid);
    }

    ///find pipeline connector uuid from pipeipe uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector
    pub fn find_pipeline_uuid_from_pipeline_connector(&mut self, uuid: Uuid) -> Option<Uuid> {
        //find uuid of pipeline connector
        return self.pipeline_connector_to_pipeline.get(&uuid).cloned(); 
    }

    /// create pipeline
    /// 
    /// # Arguments
    /// * 'from_uuid' from pipeline connector's uuid
    /// * 'to uuid' to pipeline connector's uuid
    pub fn create_pipeline(seq_id_generator: &mut SequentialIdGenerator, window_session: &mut WindowSession, from_uuid: Uuid, to_uuid: Uuid) -> Result<Uuid, &'static str> {
        //find pipeline connectors

        //create new pipeline

        //return uuid of new pipeline
        return Ok(0);
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
    pub fn new(seq_id_generator: &mut SequentialIdGenerator) -> Self {
        return PipelineComponent {
            uuid: seq_id_generator.get_id() 
        }
    }
}

//map: pipeline_connectors->parent

//could store parent uuid for connector inside connector or mapping