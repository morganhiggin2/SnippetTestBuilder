use bimap::BiHashMap;

use crate::utils::sequential_id_generator::Uuid;

pub struct VisualDirectoryComponentManager {
    directory_front_to_directory: BiHashMap<Uuid, Uuid>
}

impl Default for VisualDirectoryComponentManager {
    fn default() -> Self {
        return VisualDirectoryComponentManager { 
            directory_front_to_directory: BiHashMap::new()
        };
    }
}

impl VisualDirectoryComponentManager {
    /// find directory front uuid from directory uuid
    /// 
    /// # Arguments
    /// * 'uuid' - directory uuid
    pub fn find_directory_front_uuid(&self, uuid: &Uuid) -> Option<Uuid> {
        return self.directory_front_to_directory.get_by_right(uuid).copied(); 
    }
    
    /// find directorys uuid from directory front uuid
    ///  
    /// # Arguments
    /// * 'uuid' - directory front uuid
    pub fn find_directory_uuid(&self, uuid: &Uuid) -> Option<Uuid> {
        return self.directory_front_to_directory.get_by_left(uuid).copied(); 
    }

    /// put directory front and component pair
    /// will overwrite
    pub fn put_directory_uuid(&mut self, front_uuid: Uuid, component_uuid: Uuid) {
        self.directory_front_to_directory.insert(front_uuid, component_uuid);
    }
}