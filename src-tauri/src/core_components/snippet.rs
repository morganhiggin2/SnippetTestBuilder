use crate::{state_management::{ApplicationState, window_manager::WindowSession, external_snippet_manager::{ExternalSnippet, IOContentType}, visual_snippet_component_manager::{self, VisualSnippetComponentManager}}, utils::sequential_id_generator::{SequentialIdGenerator, self}};
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
    pipeline_connector_to_pipeline: HashMap<Uuid, Uuid>,
    pipeline_components_to_snippet: HashMap<Uuid, Uuid>

    //mapping for snippets and pipelines
    //list of uuid of snippets to index in edge adj list
    //uuid_to_edge_adj_index: HashMap<u32, usize>
    //edge adj list
}

//TODO to adapt for multi input and outputs
//have hashmap of <(from_uuid, to_uuid), pipeline_uuid>
//and bihashmap of <from_uuid, to_uuid> which only includes pipeline connectors involed in pipelines

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

pub struct PipelineComponent {
    uuid: Uuid, 
    from_pipeline_connector_uuid: Uuid,
    to_pipeline_connector_uuid: Uuid
}

//struct for the json serialization for snippet
#[derive(Serialize, Deserialize)]
pub struct FrontSnippetContent {
    id: Uuid,
    name: String,
    pipeline_connectors: Vec<FrontPipelineConnectorContent>
}

#[derive(Serialize, Deserialize)]
pub struct FrontPipelineConnectorContent {
    id: Uuid,
    name: String,
    content_type: IOContentType,
    input: bool 
}

//struct for the json serialization for pipieline
#[derive(Serialize, Deserialize)]
pub struct FrontPipelineContent {
    id: Uuid,
}

impl Default for SnippetManager {
    fn default() -> Self {
        return SnippetManager {
            snippets: HashMap::with_capacity(12),
            pipelines: HashMap::with_capacity(12),
            pipeline_connector_to_pipeline: HashMap::with_capacity(24),
            pipeline_components_to_snippet: HashMap::with_capacity(24)
            //uuid_to_edge_adj_index: HashMap::with_capacity(24),
        };
    }
}

impl SnippetManager {
    /// create a new snippet
    pub fn new_snippet(&mut self, seq_id_generator: &mut SequentialIdGenerator, external_snippet: &ExternalSnippet) -> Uuid {
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
            self.pipeline_components_to_snippet.insert(pipeline_connector.get_uuid(), snippet_uuid);
        }

        //move pipeline connectors to snippet component
        snippet_component.pipeline_connectors = pipeline_connectors;

        //add to snippets list in snippet manager
        self.snippets.insert(snippet_uuid, snippet_component);

        //return uuid of snippet
        return snippet_uuid;
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

    /// find snippet from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the snippet 
    pub fn find_pipeline(&self, uuid: &Uuid) -> Option<&PipelineComponent>{
        //find pipeline in vector
        return self.pipelines.get(uuid);
    }

    /// find mutable reference to snippet from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the snippet 
    pub fn find_pipeline_mut(&mut self, uuid: &Uuid) -> Option<&mut PipelineComponent>{
        //find pipeline in vector
        return self.pipelines.get_mut(uuid);
    }
    
    /// find pipeline connector uuid from pipeipe uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector coming out of pipeline
    pub fn find_pipeline_uuid_from_pipeline_connector(&self, uuid: &Uuid) -> Option<Uuid> {
        //find uuid of pipeline connector
        return self.pipeline_connector_to_pipeline.get(uuid).cloned(); 
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
    pub fn create_pipeline(&mut self,  seq_id_generator: &mut SequentialIdGenerator, from_uuid: Uuid, to_uuid: Uuid) -> Result<Uuid, &'static str> {
        //get valid direction of pipeline, as from_uuid and to_uuid are not guarnteed to be input:false -> input:true
        let mut from_uuid = from_uuid;
        let mut to_uuid = to_uuid;

        {
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
            let to_pipeline_connector = match to_snippet.find_pipeline_connector(to_uuid) {
                Some(result) => result,
                None => {
                    return Err("to pipeline connector does not exist in snippet");
                }
            };

            //if it is flowing input -> output
            if (from_pipeline_connector.get_input() && !to_pipeline_connector.get_input()) {
                //revert order
                let temp = to_uuid;
                to_uuid = from_uuid;
                from_uuid = temp;
            }
        }

        //create new pipeline
        let pipeline_component = PipelineComponent::new(seq_id_generator, &from_uuid, &to_uuid);

        //get pipeline uuid for return
        let pipeline_uuid = pipeline_component.get_uuid();

        //add pipeline connecting values to snippet manager
        self.pipeline_connector_to_pipeline.insert(from_uuid, pipeline_uuid);
        self.pipeline_connector_to_pipeline.insert(to_uuid, pipeline_uuid);

        //add to snippet manager
        self.pipelines.insert(pipeline_uuid, pipeline_component);

        //return uuid of new pipeline
        return Ok(pipeline_uuid);
    }

    /// deletes pipeline and all snippet manager internal links
    /// upon unsuccessfull deletion, deletes taken before in this method are not reversed
    /// 
    /// # Arguments 
    /// * 'uuid' - uuid of the pipeline component
    pub fn delete_pipeline(&mut self, uuid: &Uuid) -> Result<(), &str> {
        let mut from_pipeline_connector_uuid = 0;
        let mut to_pipeline_connector_uuid = 0;

        //get pipeline uuids without violating borrow rules for self
        {
            //get pipeline
            let pipeline_component = match self.find_pipeline(uuid) {
                Some(result) => result,
                None => {
                    return Err("pipeline does not exist in snippet manager to delete"); 
                }
            };

            //grab copies of the pipeline connector uuids
            from_pipeline_connector_uuid = pipeline_component.from_pipeline_connector_uuid.clone();
            to_pipeline_connector_uuid = pipeline_component.to_pipeline_connector_uuid.clone();
        }

        //delete front from pipeline connector to pipeline relationship 
        match self.pipeline_connector_to_pipeline.remove(&from_pipeline_connector_uuid) {
            Some(_) => (),
            None => {
                return Err("front from pipeline connector does not exist in pipeline connector to pipeline relationship");
            }
        };
 
        //delete front to pipeline connector to pipeline relationship 
        match self.pipeline_connector_to_pipeline.remove(&to_pipeline_connector_uuid) {
            Some(_) => (),
            None => {
                return Err("front to pipeline connector does not exist in pipeline connector to pipeline relationship");
            }
        };       

        //delete pipeline in snippet manager
        match self.pipelines.remove(uuid) {
            Some(_) => (),
            None => {
                return Err("pipeline does not exist in snippet manager");
            }
        }

        return Ok(());
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
        let to_pipeline_connector = match to_snippet.find_pipeline_connector(to_uuid) {
            Some(result) => result,
            None => {
                return Err("to pipeline connector does not exist in snippet");
            }
        };

        {
            //verify that a connection between the two same does not already exist
            let from_result = match self.find_pipeline_uuid_from_pipeline_connector(&from_uuid) {
                Some(_) => true,
                None => false
            };

            let to_result = match self.find_pipeline_uuid_from_pipeline_connector(&to_uuid) {
                Some(_) => true,
                None => false
            };

            if from_result || to_result {
                return Ok(false);
            }
        }

        //verify that one is an output and one is an input
        {
            if from_pipeline_connector.get_input() == to_pipeline_connector.get_input() {
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


    pub fn check_pipeline_connector_capacity_full(&self, pipeline_connector_uuid: &Uuid) -> bool {
        //check if in pipeline connector map
        return match self.find_pipeline_uuid_from_pipeline_connector(pipeline_connector_uuid) {
            Some(_) => true,
            None => false 
        };
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
    pub fn get_snippet_to_front_snippet(&self, visual_snippet_component_manager: &mut VisualSnippetComponentManager, seq_id_generator: &mut SequentialIdGenerator, snippet_manager: &SnippetManager) -> FrontSnippetContent {
        //get pipeline connectors as front pipeline connectors
        let front_pipeline_connectors = self.get_pipeline_connectors_as_pipeline_connector_content(visual_snippet_component_manager, seq_id_generator);

        //create front snippet
        let front_snippet = FrontSnippetContent::new(
            visual_snippet_component_manager,
            seq_id_generator.get_id(),
            self.get_name(),
            self.get_uuid(),
            front_pipeline_connectors
        );

        return front_snippet;
    }

    fn get_pipeline_connectors_as_pipeline_connector_content(&self, visual_snippet_component_manager: &mut VisualSnippetComponentManager, seq_id_generator: &mut SequentialIdGenerator) -> Vec<FrontPipelineConnectorContent> {
        //generate contents vector
        let mut contents: Vec<FrontPipelineConnectorContent> = Vec::with_capacity(self.pipeline_connectors.len());

        //get io points
        for pipeline_connector in self.pipeline_connectors.iter() {
            //push to contents
            contents.push(
                FrontPipelineConnectorContent::new(
                    visual_snippet_component_manager,
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

    pub fn get_input(&self) -> bool {
        return self.input;
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

    pub fn get_pipeline_as_front_content(&self, visual_snippet_component_manager: &mut VisualSnippetComponentManager, seq_id_generator: &mut SequentialIdGenerator) -> FrontPipelineContent {

        let front_pipeline_content = FrontPipelineContent {
            id: seq_id_generator.get_id()
        };

        visual_snippet_component_manager.put_pipeline(front_pipeline_content.id, self.uuid);

        return front_pipeline_content;
    }
}


impl FrontSnippetContent {
    pub fn new(visual_snippet_component_manager: &mut VisualSnippetComponentManager, uuid: Uuid, name: String, internal_id: Uuid, pipeline_connectors: Vec<FrontPipelineConnectorContent>) -> Self {
        let front_content = FrontSnippetContent {
            id: uuid,
            name: name,
            pipeline_connectors: pipeline_connectors 
        };

        //add front content to visual component manager
        visual_snippet_component_manager.put_snippet(uuid, internal_id);

        return front_content;
    }
}

impl FrontPipelineConnectorContent {
    pub fn new(visual_snippet_component_manager: &mut VisualSnippetComponentManager, uuid: Uuid, pipeline_connector_id: Uuid, name: String, content_type: IOContentType, input: bool) -> Self {
        let front_content = FrontPipelineConnectorContent {
            id: uuid,
            name: name,
            content_type: content_type,
            input: input
        };

        println!("{}, {}", uuid, pipeline_connector_id);

        //add front content to visual component manager
        visual_snippet_component_manager.put_pipeline_connector(uuid, pipeline_connector_id);

        return front_content;
    }
    
}

impl FrontPipelineContent {
    pub fn new(visual_snippet_component_manager: &mut VisualSnippetComponentManager, uuid: Uuid, pipeline_uuid: Uuid) -> Self {
        let front_content = FrontPipelineContent {
            id: uuid,
        };

        //add front content to visual compoennt manager
        visual_snippet_component_manager.put_pipeline(uuid, pipeline_uuid);

        return front_content;
    }
}
//map: pipeline_connectors->parent

//could store parent uuid for connector inside connector or mapping
//TODO change alot of impl from getting self seperatrly to directory getting &self or &mut self
//TODO implement automatically the order of pipeline so that input: false always goes to input:true, also add validation for this to make sure
//  that they appose
//TODO visual grid, and gridlocking



//TODO rust backend visual to id mapper 
//TODO convert all then()'s to .await
//TODO hide all new shippet stuff with internal id's behind abstraction layer of virtual component manager
//TODO be able to move visual snippet components on screen, have pipelines either (a disapear temproarly, so visiabiliy off, then redraw), or if working
//  b) be able to have dotted lines that move with the snippet as the pipelines when it moves (can implement a first, then b)
