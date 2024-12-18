use bimap::BiHashMap;
use serde::{Deserialize, Serialize};

use crate::utils::sequential_id_generator::Uuid;

pub struct VisualSnippetComponentManager {
    pipeline_front_to_pipeline: BiHashMap<Uuid, Uuid>,
    pipeline_connector_front_to_pipeline_connector: BiHashMap<Uuid, Uuid>,
    snippet_front_to_snippet: BiHashMap<Uuid, Uuid>,
    parameter_front_to_parameter: BiHashMap<Uuid, Uuid>,
}

//struct for the json serialization for snippet
#[derive(Serialize, Deserialize)]
pub struct FrontSnippetContent {
    id: Uuid,
    name: String,
    pipeline_connectors: Vec<FrontPipelineConnectorContent>,
    parameters: Vec<FrontParameterContent>,
}

#[derive(Serialize, Deserialize)]
pub struct FrontPipelineConnectorContent {
    id: Uuid,
    name: String,
    input: bool,
}

//struct for the json serialization for pipieline
#[derive(Serialize, Deserialize)]
pub struct FrontPipelineContent {
    id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct FrontParameterContent {
    id: Uuid,
    name: String,
    p_type: String,
}

impl Default for VisualSnippetComponentManager {
    fn default() -> Self {
        return VisualSnippetComponentManager {
            pipeline_front_to_pipeline: BiHashMap::new(),
            pipeline_connector_front_to_pipeline_connector: BiHashMap::new(),
            snippet_front_to_snippet: BiHashMap::new(),
            parameter_front_to_parameter: BiHashMap::new(),
        };
    }
}

impl VisualSnippetComponentManager {
    /// find pipeline front uuid from pipeline uuid
    ///
    /// # Arguments
    /// * 'uuid' - pipeline uuid
    pub fn find_pipeline_front_uuid(&self, uuid: &Uuid) -> Option<Uuid> {
        return self.pipeline_front_to_pipeline.get_by_right(uuid).copied();
    }

    /// find pipelines uuid from pipeline front uuid
    ///
    /// # Arguments
    /// * 'uuid' - pipeline front uuid
    pub fn find_pipeline_uuid(&self, uuid: &Uuid) -> Option<Uuid> {
        return self.pipeline_front_to_pipeline.get_by_left(uuid).copied();
    }

    /// find pipeline connector front uuid from pipeline connector uuid
    ///
    /// # Arguments
    /// * 'uuid' - pipeline uuid
    pub fn find_pipeline_connector_front_uuid(&self, uuid: &Uuid) -> Option<Uuid> {
        return self
            .pipeline_connector_front_to_pipeline_connector
            .get_by_right(uuid)
            .copied();
    }

    /// find pipelines connector uuid from pipeline connector front uuid
    ///
    /// # Arguments
    /// * 'uuid' - pipeline front uuid
    pub fn find_pipeline_connector_uuid(&self, uuid: &Uuid) -> Option<Uuid> {
        return self
            .pipeline_connector_front_to_pipeline_connector
            .get_by_left(uuid)
            .copied();
    }

    /// find snippet front uuid from snippet uuid
    ///
    /// # Arguments
    /// * 'uuid' - pipeline uuid
    pub fn find_snippet_front_uuid(&self, uuid: &Uuid) -> Option<Uuid> {
        return self.snippet_front_to_snippet.get_by_right(uuid).copied();
    }

    /// find snippet uuid from snippet front uuid
    ///
    /// # Arguments
    /// * 'uuid' - pipeline front uuid
    pub fn find_snippet_uuid(&self, uuid: &Uuid) -> Option<Uuid> {
        return self.snippet_front_to_snippet.get_by_left(uuid).copied();
    }

    /// find parameter front uuid from parameter uuid
    pub fn find_parameter_front_uuid(&self, uuid: &Uuid) -> Option<Uuid> {
        return self
            .parameter_front_to_parameter
            .get_by_right(uuid)
            .copied();
    }

    /// put pipeline front and component pair
    /// will overwrite
    pub fn put_pipeline(&mut self, front_uuid: Uuid, component_uuid: Uuid) {
        self.pipeline_front_to_pipeline
            .insert(front_uuid, component_uuid);
    }

    /// deletes pipeline from front pipeline component uuid
    ///
    /// # Arguments
    /// * 'uuid' - uuid of the front pipeline component
    pub fn delete_pipeline_by_pipeline_front(&mut self, uuid: &Uuid) -> Result<(), &'static str> {
        match self.pipeline_front_to_pipeline.remove_by_left(uuid) {
            Some(_) => {
                return Ok(());
            }
            None => {
                return Err("front uuid does not exist in virtual component pipeline relationship")
            }
        }
    }

    /// deletes pipeline from pipeline uuid
    ///
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline
    pub fn delete_pipeline_by_pipeline(&mut self, uuid: &Uuid) -> Result<(), &'static str> {
        match self.pipeline_front_to_pipeline.remove_by_left(uuid) {
            Some(_) => {
                return Ok(());
            }
            None => {
                return Err("front uuid does not exist in virtual component pipeline relationship")
            }
        }
    }

    /// put pipeline connector front and component pair
    /// will overwrite
    pub fn put_pipeline_connector(&mut self, front_uuid: Uuid, component_uuid: Uuid) {
        self.pipeline_connector_front_to_pipeline_connector
            .insert(front_uuid, component_uuid);
    }

    /// delete pipeline connector front with front uuid
    pub fn delete_pipeline_connector_by_front(
        &mut self,
        front_uuid: &Uuid,
    ) -> Result<(), &'static str> {
        match self
            .pipeline_connector_front_to_pipeline_connector
            .remove_by_left(front_uuid)
        {
            Some(_) => {
                return Ok(());
            }
            None => {
                return Err("cannot delete pipeline connector front from front uuid, as pipeline connector front does not exist");
            }
        };
    }

    /// delete pipeline connector front with internal uuid
    pub fn delete_pipeline_connector_by_internal(
        &mut self,
        internal_uuid: &Uuid,
    ) -> Result<(), &'static str> {
        match self
            .pipeline_connector_front_to_pipeline_connector
            .remove_by_right(internal_uuid)
        {
            Some(_) => {
                return Ok(());
            }
            None => {
                return Err("cannot delete pipeline connector front from internal uuid, as pipeline connector front does not exist");
            }
        };
    }

    /// put snippet front and component pair
    /// will overwrite
    pub fn put_snippet(&mut self, front_uuid: Uuid, component_uuid: Uuid) {
        self.snippet_front_to_snippet
            .insert(front_uuid, component_uuid);
    }

    /// will delete snippet front with fron tuuid
    pub fn delete_snippet_by_front(&mut self, front_uuid: Uuid) -> Result<(), &'static str> {
        match self.snippet_front_to_snippet.remove_by_left(&front_uuid) {
            Some(_) => {
                return Ok(());
            }
            None => {
                return Err(
                    "cannot delete snippet front from front uuid, as snippet front does not exist",
                );
            }
        };
    }

    /// will delete snippet front with fron tuuid
    pub fn delete_snippet_by_internal(&mut self, internal_uuid: &Uuid) -> Result<(), &'static str> {
        match self.snippet_front_to_snippet.remove_by_right(internal_uuid) {
            Some(_) => {
                return Ok(());
            }
            None => {
                return Err("cannot delete snippet front from internal uuid, as snippet front does not exist");
            }
        };
    }

    /// put parameter frontinto parameter
    pub fn put_parameter(&mut self, front_uuid: Uuid, parameter_uuid: Uuid) {
        self.parameter_front_to_parameter
            .insert(front_uuid, parameter_uuid);
    }

    /// get snippet uuid from parameter
    pub fn find_parameter_uuid_from_parameter_front(
        &self,
        parameter_front_uuid: Uuid,
    ) -> Option<Uuid> {
        return self
            .parameter_front_to_parameter
            .get_by_left(&parameter_front_uuid)
            .copied();
    }
}

impl FrontSnippetContent {
    pub fn new(
        visual_snippet_component_manager: &mut VisualSnippetComponentManager,
        uuid: Uuid,
        name: String,
        internal_id: Uuid,
        pipeline_connectors: Vec<FrontPipelineConnectorContent>,
        parameters: Vec<FrontParameterContent>,
    ) -> Self {
        let front_content = FrontSnippetContent {
            id: uuid,
            name: name,
            pipeline_connectors: pipeline_connectors,
            parameters: parameters,
        };

        //add front content to visual component manager
        visual_snippet_component_manager.put_snippet(uuid, internal_id);

        return front_content;
    }
}

impl FrontPipelineConnectorContent {
    pub fn new(
        visual_snippet_component_manager: &mut VisualSnippetComponentManager,
        uuid: Uuid,
        pipeline_connector_id: Uuid,
        name: String,
        input: bool,
    ) -> Self {
        let front_content = FrontPipelineConnectorContent {
            id: uuid,
            name: name,
            input: input,
        };

        //add front content to visual component manager
        visual_snippet_component_manager.put_pipeline_connector(uuid, pipeline_connector_id);

        return front_content;
    }
}

impl FrontPipelineContent {
    pub fn new(
        visual_snippet_component_manager: &mut VisualSnippetComponentManager,
        uuid: Uuid,
        pipeline_uuid: Uuid,
    ) -> Self {
        let front_content = FrontPipelineContent { id: uuid };

        //add front content to visual component manager
        visual_snippet_component_manager.put_pipeline(uuid, pipeline_uuid);

        return front_content;
    }

    pub fn get_uuid(&self) -> Uuid {
        return self.id;
    }
}

impl FrontParameterContent {
    pub fn new(
        visual_directory_component_manager: &mut VisualSnippetComponentManager,
        uuid: Uuid,
        parameter_uuid: Uuid,
        name: String,
        p_type: String,
    ) -> Self {
        let front_content = FrontParameterContent {
            id: uuid,
            name: name,
            p_type: p_type,
        };

        // add front content to visual component manager
        visual_directory_component_manager.put_parameter(uuid, parameter_uuid);

        return front_content;
    }
}
