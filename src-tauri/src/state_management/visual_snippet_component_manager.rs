use bimap::BiHashMap;

use crate::utils::sequential_id_generator::Uuid;

pub struct VisualSnippetComponentManager {
    pipeline_front_to_pipeline: BiHashMap<Uuid, Uuid>,
    pipeline_connector_front_to_pipeline_connector: BiHashMap<Uuid, Uuid>,
    snippet_front_to_snippet: BiHashMap<Uuid, Uuid>
}

impl Default for VisualSnippetComponentManager {
    fn default() -> Self {
        return VisualSnippetComponentManager { 
            pipeline_front_to_pipeline: BiHashMap::new(),
            pipeline_connector_front_to_pipeline_connector: BiHashMap::new(),
            snippet_front_to_snippet: BiHashMap::new()
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
        return self.pipeline_connector_front_to_pipeline_connector.get_by_right(uuid).copied(); 
    }
    
    /// find pipelines connector uuid from pipeline connector front uuid
    ///  
    /// # Arguments
    /// * 'uuid' - pipeline front uuid
    pub fn find_pipeline_connector_uuid(&self, uuid: &Uuid) -> Option<Uuid> {
        return self.pipeline_connector_front_to_pipeline_connector.get_by_left(uuid).copied(); 
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

    /// put pipeline front and component pair
    /// will overwrite
    pub fn put_pipeline(&mut self, front_uuid: Uuid, component_uuid: Uuid) {
        self.pipeline_front_to_pipeline.insert(front_uuid, component_uuid);
    }

    /// deletes pipeline from front pipeline component uuid 
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the front pipeline component
    pub fn delete_pipeline_by_pipeline_front(&mut self, uuid: &Uuid) -> Result<(), &str> {
        match self.pipeline_front_to_pipeline.remove_by_left(uuid) {
            Some(result) => {
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
    pub fn delete_pipeline_by_pipeline(&mut self, uuid: &Uuid) -> Result<(), &str> {
        match self.pipeline_front_to_pipeline.remove_by_left(uuid) {
            Some(result) => {
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
        self.pipeline_connector_front_to_pipeline_connector.insert(front_uuid, component_uuid);
    }

    /// put snippet front and component pair
    /// will overwrite
    pub fn put_snippet(&mut self, front_uuid: Uuid, component_uuid: Uuid) {
        self.snippet_front_to_snippet.insert(front_uuid, component_uuid);
    }

}