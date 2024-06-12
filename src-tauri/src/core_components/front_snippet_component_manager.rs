use crate::{state_management::{ApplicationState, window_manager::WindowSession, external_snippet_manager::{ExternalSnippet, IOContentType}, visual_snippet_component_manager::{self, VisualSnippetComponentManager}}, utils::sequential_id_generator::{SequentialIdGenerator, self}};
use serde::{Serialize, Deserialize};
use crate::utils::sequential_id_generator::Uuid;


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

    pub fn get_uuid(&self) -> Uuid {
        return self.id;
    }
}
