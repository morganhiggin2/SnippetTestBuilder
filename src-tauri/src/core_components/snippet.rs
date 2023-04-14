use crate::{state_management::{ApplicationState, window_manager::WindowSession, external_snippet_manager::{ExternalSnippet, IOContentType}}, utils::sequential_id_generator::{SequentialIdGenerator, self}};
use std::{sync::MutexGuard, collections::HashMap};
use bimap::BiHashMap;
use serde::{Serialize, Deserialize};
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
    pipeline_components_to_snippet: HashMap<Uuid, Uuid>

    //mapping for snippets and pipelines
    //list of uuid of snippets to index in edge adj list
    //uuid_to_edge_adj_index: HashMap<u32, usize>
    //edge adj list
}

/// the actual snippet itself
pub struct SnippetComponent {
    uuid: Uuid,
    name: String,
    external_snippet_uuid: Uuid,
    pipeline_connectors: Vec<PipelineConnectorComponent> 
}

pub struct PipelineConnectorComponent {
    uuid: Uuid,
    external_pipeline_connector_uuid: Uuid,
    name: String,
    content_type: IOContentType,
    input: bool
}

struct PipelineComponent {
    uuid: Uuid, 
    from_pipeline_connector_uuid: Uuid,
    to_pipeline_connector_uuid: Uuid
}

//struct for the josn serialization
#[derive(Serialize, Deserialize)]
pub struct FrontSnippetContent {
    id: Uuid,
    name: String,
    internal_id: Uuid,
    pipeline_connectors: Vec<FrontPipelineConnectorContent>
}

#[derive(Serialize, Deserialize)]
pub struct FrontPipelineConnectorContent {
    id: Uuid,
    pipeline_connector_id: Uuid,
    name: String,
    content_type: IOContentType,
    input: bool 
}

impl Default for SnippetManager {
    fn default() -> Self {
        return SnippetManager {
            snippets: HashMap::with_capacity(12),
            pipelines: HashMap::with_capacity(12),
            from_pipeline_connector_to_pipeline: HashMap::with_capacity(24),
            to_pipeline_connector_to_pipeline: HashMap::with_capacity(24),
            pipeline_components_to_snippet: HashMap::with_capacity(24)
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

        //get snippet name
        let snippet_name : String = external_snippet.get_name();

        //add components from external snippet to snippet
        snippet_component.external_snippet_uuid = external_snippet.get_uuid();
        snippet_component.name = snippet_name;

        //add io points to snippet component as pipeline connectors
        let pipeline_connectors = external_snippet.get_io_points_as_pipeline_connectors(seq_id_generator);

        //add pipeline connector uuid to snippet mapping
        for pipeline_connector in pipeline_connectors.iter() {
            println!("{}", pipeline_connector.get_uuid());
            window_session.snippet_manager.pipeline_components_to_snippet.insert(pipeline_connector.get_uuid(), snippet_uuid);
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
    pub fn find_snippet(&self, uuid: &Uuid) -> Option<&SnippetComponent>{
        //find pipeline in vector
        return self.snippets.get(uuid);
    }

    /// find mutable reference to snippet from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the snippet 
    pub fn find_snippet_mut(&mut self, uuid: &Uuid) -> Option<&mut SnippetComponent>{
        //find pipeline in vector
        return self.snippets.get_mut(uuid);
    }

    /// find pipeline connector uuid from pipeipe uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector going into pipeline
    pub fn find_pipeline_uuid_from_from_pipeline_connector(&self, uuid: &Uuid) -> Option<Uuid> {
        //find uuid of pipeline connector
        return self.from_pipeline_connector_to_pipeline.get(uuid).cloned(); 
    }
    
    /// find pipeline connector uuid from pipeipe uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector coming out of pipeline
    pub fn find_pipeline_uuid_from_to_pipeline_connector(&self, uuid: &Uuid) -> Option<Uuid> {
        //find uuid of pipeline connector
        return self.to_pipeline_connector_to_pipeline.get(uuid).cloned(); 
    }
    /// find snippet uuid from pipeline uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector 
    pub fn find_snippet_uuid_from_pipeline_connector(&self, uuid: &Uuid) -> Option<Uuid> {
        return self.pipeline_components_to_snippet.get(uuid).cloned();
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

    /// validate pipeline
    /// returning weither or not this is a valid pipeline creation
    /// assumed by error that the pipeline connectors their
    /// underying snippets exist
    /// 
    /// # Arguments
    /// * 'from_uuid' from pipeline connector's uuid
    /// * 'to uuid' to pipeline connector's uuid
    pub fn validate_pipeline(&self, from_uuid: Uuid, to_uuid: Uuid) -> Result<bool, &'static str> {
        //find pipeline connectors
        //find pipeline connector uuid
        let from_snippet_uuid = match self.find_snippet_uuid_from_pipeline_connector(&from_uuid) {
            Some(result) => result,
            None => {
                return Err("from snippet uuid from pipeline connector not found");
            }
        };

        let from_snippet = match self.find_snippet(&from_snippet_uuid) {
            Some(result) => result,
            None => {
                return Err("from snippet from pipeline connector not found")
            }
        };

        //verify pipeline connector exists in pipeline
        let from_pipeline_connector = match from_snippet.find_pipeline_connector(from_uuid) {
            Some(result) => result,
            None => {
                return Err("from pipeline connector does not exist in snippet");
            }
        };

        //find pipeline connector uuid
        let to_snippet_uuid = match self.find_snippet_uuid_from_pipeline_connector(&to_uuid) {
            Some(result) => result,
            None => {
                return Err("to snippet uuid from pipeline connector not found");
            }
        };

        let to_snippet = match self.find_snippet(&to_snippet_uuid) {
            Some(result) => result,
            None => {
                return Err("to snippet from pipeline connector not found")
            }
        };

        //verify pipeline connector exists in pipeline
        let to_pipeline_connector = match to_snippet.find_pipeline_connector(from_uuid) {
            Some(result) => result,
            None => {
                return Err("to pipeline connector does not exist in snippet");
            }
        };

        {
            //verify that a connection between the two same does not already exist
            let from_result = match self.find_pipeline_uuid_from_from_pipeline_connector(&from_uuid) {
                Some(_) => true,
                None => false
            };

            let to_result = match self.find_pipeline_uuid_from_to_pipeline_connector(&to_uuid) {
                Some(_) => true,
                None => false
            };

            if !from_result && !to_result {
                return Ok(false);
            }
        }

        //verify that the connection is between different snippets
        if from_snippet.get_uuid() == to_snippet.get_uuid() {
            return Ok(false);
        }

        //verify types match
        if from_pipeline_connector.get_type() != to_pipeline_connector.get_type() {
            return Ok(false); 
        }

        return Ok(true);
    }
}

impl SnippetComponent {
    pub fn new(seq_id_generator: &mut SequentialIdGenerator) -> Self {
        return SnippetComponent {
            uuid: seq_id_generator.get_id(),
            name: String::new(),
            external_snippet_uuid: 0,
            pipeline_connectors: Vec::new()
        }
    }

    /// find mutable refernece to pipeline connector from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector
    fn find_pipeline_connector(&self, uuid: Uuid) -> Option<&PipelineConnectorComponent>{
        //find pipeline in vector
        return self.pipeline_connectors.iter().find(|pipe: &&PipelineConnectorComponent| pipe.uuid == uuid);
    }

    /// find mutable refernece to pipeline connector from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector
    fn find_pipeline_connector_mut(&mut self, uuid: Uuid) -> Option<&mut PipelineConnectorComponent>{
        //find pipeline in vector
        return self.pipeline_connectors.iter_mut().find(|pipe: &&mut PipelineConnectorComponent| pipe.uuid == uuid);
    }

    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    /// get snippets as front snippet content
    pub fn get_snippet_to_front_snippet(&self, seq_id_generator: &mut SequentialIdGenerator, snippet_manager: &SnippetManager) -> FrontSnippetContent {
        //get pipeline connectors as front pipeline connectors
        let front_pipeline_connectors = self.get_pipeline_connectors_as_pipeline_connector_content(seq_id_generator);

        //create front snippet
        let front_snippet = FrontSnippetContent::new(
            seq_id_generator.get_id(),
            self.get_name(),
            self.get_uuid(),
            front_pipeline_connectors
        );

        return front_snippet;
    }

    fn get_pipeline_connectors_as_pipeline_connector_content(&self, seq_id_generator: &mut SequentialIdGenerator) -> Vec<FrontPipelineConnectorContent> {
        //generate contents vector
        let mut contents: Vec<FrontPipelineConnectorContent> = Vec::with_capacity(self.pipeline_connectors.len());

        //get io points
        for pipeline_connector in self.pipeline_connectors.iter() {
            //push to contents
            contents.push(
                FrontPipelineConnectorContent::new(
                    seq_id_generator.get_id(),
                    pipeline_connector.uuid.clone(),
                    pipeline_connector.name.clone(),
                    pipeline_connector.content_type.clone(),
                    pipeline_connector.input
                )
            )
        }

        return contents;
    }

}

impl PipelineConnectorComponent {
    pub fn new(seq_id_generator: &mut SequentialIdGenerator, external_pipeline_connector_uuid: Uuid, name: &str, content_type: &IOContentType, input: bool) -> Self {
        return PipelineConnectorComponent {
            uuid: seq_id_generator.get_id(),
            external_pipeline_connector_uuid: external_pipeline_connector_uuid,
            name: name.clone().to_string(),
            content_type: content_type.clone(),
            input: input
        }
    }

    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }

    pub fn get_type(&self) -> IOContentType {
        return self.content_type.clone();
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


impl FrontSnippetContent {
    pub fn new(id: Uuid, name: String, internal_id: Uuid, pipeline_connectors: Vec<FrontPipelineConnectorContent>) -> Self {
        return FrontSnippetContent {
            id: id,
            name: name,
            internal_id: internal_id,
            pipeline_connectors: pipeline_connectors 
        }
    }
}

impl FrontPipelineConnectorContent {
    pub fn new(id: Uuid, pipeline_connector_id: Uuid, name: String, content_type: IOContentType, input: bool) -> Self {
        return FrontPipelineConnectorContent {
            id: id,
            pipeline_connector_id: pipeline_connector_id,
            name: name,
            content_type: content_type,
            input: input
        }
    }
    
}
//map: pipeline_connectors->parent

//could store parent uuid for connector inside connector or mapping
//TODO change alot of impl from getting self seperatrly to directory getting &self or &mut self
//TODO implement automatically the order of pipeline so that input: false always goes to input:true, also add validation for this to make sure
//that they appose