use bimap::BiHashMap;
use serde::{Deserialize, Serialize};

use crate::{state_management::external_snippet_manager::ExternalSnippetManager, utils::sequential_id_generator::{SequentialIdGenerator, Uuid}};

use super::directory_manager::{DirectoryManager, ExternalSnippetFileContainer};

//TODO link with front directory component manager
pub struct VisualDirectoryComponentManager {
    directory_front_to_snippet_file_container: BiHashMap<Uuid, Uuid>
}

impl Default for VisualDirectoryComponentManager {
    fn default() -> Self {
        return VisualDirectoryComponentManager { 
            directory_front_to_snippet_file_container: BiHashMap::new()
        };
    }
}

//struct for the josn serialization
#[derive(Serialize, Deserialize)]
pub struct FrontDirectoryContent {
    id: Uuid,
    name: String,
    file_type: FrontDirectoryContentType,
    is_directory: bool,
    level: u32,
    showing: bool,
}

#[derive(Serialize, Deserialize)]
pub enum FrontDirectoryContentType {
    Directory,
    Snippet
}

impl VisualDirectoryComponentManager {
    /// find directory front uuid from directory uuid
    /// 
    /// # Arguments
    /// * 'uuid' - snippet file container uuid
    pub fn find_directory_front_uuid(&self, uuid: &Uuid) -> Option<Uuid> {
        return self.directory_front_to_snippet_file_container.get_by_right(uuid).copied(); 
    }
    
    /// find directorys uuid from directory front uuid
    ///  
    /// # Arguments
    /// * 'uuid' - directory front uuid
    pub fn find_snippet_file_container_uuid(&self, uuid: &Uuid) -> Option<Uuid> {
        return self.directory_front_to_snippet_file_container.get_by_left(uuid).copied(); 
    }

    /// put directory front and component pair
    /// will overwrite
    pub fn put_snippet_file_container_uuid(&mut self, front_uuid: Uuid, component_uuid: Uuid) {
        self.directory_front_to_snippet_file_container.insert(front_uuid, component_uuid);
    }
}

impl FrontDirectoryContent {
    pub fn new(uuid: Uuid, name: String, internal_id: Uuid, file_type: FrontDirectoryContentType, is_directory: bool, level: u32, showing: bool) -> Self {
        let front_content = FrontDirectoryContent {
            id: uuid,
            name: name,
            file_type: file_type,
            is_directory: is_directory,
            level: level,
            showing: showing,
        };

        return front_content;
    }

    /// create new front snippet content of type external snippet file container 
    pub fn new_snippet(directory_manager: &DirectoryManager, external_snippet_manager: &ExternalSnippetManager, seq_id_generator: &mut SequentialIdGenerator, external_snippet_file_container: &ExternalSnippetFileContainer, level: u32) -> Result<Self, String> {
        //TODO: ask why &str instead of String is not working
        //call front method on file container
        let front_external_snippet_content = match external_snippet_file_container.get_as_front_content(external_snippet_manager, seq_id_generator, level) {
            Ok(result) => result,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        //add front content to visual component manager
        directory_manager.visual_component_manager.put_snippet_file_container_uuid(front_external_snippet_content.id, external_snippet_file_container.get_uuid()); 

        return Ok(front_external_snippet_content);

        //TODO delete these comments
        //find external snippet
        //let external_snippet = external_snippet_manager.find_external_snippet(external_snippet_container.get_external_snippet_uuid()).unwrap();

        //return external_snippet.get_as_front_content(visual_directory_component_manager, external_snippet_manager, seq_id_generator, level);        
    }

    /// create new front snippet content of type category 
    pub fn new_category(visual_directory_component_manager: &mut VisualDirectoryComponentManager, seq_id_generator: &mut SequentialIdGenerator, name: String,  level: u32) -> Self {
        return FrontDirectoryContent::new(
            seq_id_generator.get_id(),
            name.clone(),
            0,
            FrontDirectoryContentType::Directory,
            true,
            level,
            false,
        );
    }
}

