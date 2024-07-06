use bimap::BiHashMap;
use serde::{Deserialize, Serialize};

use crate::{state_management::external_snippet_manager::ExternalSnippetManager, utils::sequential_id_generator::{SequentialIdGenerator, Uuid}};

use super::directory_manager::{DirectoryManager, SnippetDirectoryCategory, SnippetDirectoryEntry, SnippetDirectorySnippet, SnippetDirectoryType};

//TODO link with front directory component manager
pub struct VisualDirectoryComponentManager {
    directory_front_to_directory_entry: BiHashMap<Uuid, Uuid>
}

impl Default for VisualDirectoryComponentManager {
    fn default() -> Self {
        return VisualDirectoryComponentManager { 
            directory_front_to_directory_entry: BiHashMap::new()
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
        return self.directory_front_to_directory_entry.get_by_right(uuid).copied(); 
    }
    
    /// find directorys uuid from directory front uuid
    ///  
    /// # Arguments
    /// * 'uuid' - directory front uuid
    pub fn find_directory_entry_uuid(&self, uuid: &Uuid) -> Option<Uuid> {
        return self.directory_front_to_directory_entry.get_by_left(uuid).copied(); 
    }
}

impl FrontDirectoryContent {
    pub fn new(uuid: Uuid, name: String, file_type: FrontDirectoryContentType, is_directory: bool, level: u32, showing: bool) -> Self {
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

    pub fn new_from_directory_entry(directory_entry: &SnippetDirectoryEntry, level: u32, visual_directory_component_manager: &mut VisualDirectoryComponentManager, sequential_id_generator: &mut SequentialIdGenerator) -> FrontDirectoryContent {
        let name = directory_entry.get_name();
        let directory_entry_uuid = directory_entry.get_uuid();

        match directory_entry.get_inner_as_ref() {
            SnippetDirectoryType::Category(some) => {
                return FrontDirectoryContent::new_category_from_directory_entry(name, directory_entry_uuid, some, level, visual_directory_component_manager, sequential_id_generator);
            },
            SnippetDirectoryType::Snippet(some) => {
                return FrontDirectoryContent::new_snippet_from_directory_entry(name, directory_entry_uuid, some, level, visual_directory_component_manager, sequential_id_generator)
            },
        }
    }

    fn new_snippet_from_directory_entry(name: String, directory_entry_uuid: Uuid, directory_entry: &SnippetDirectorySnippet, level: u32, visual_directory_component_manager: &mut VisualDirectoryComponentManager, sequential_id_generator: &mut SequentialIdGenerator) -> FrontDirectoryContent {
        let front_directory_content = FrontDirectoryContent {
            id: sequential_id_generator.get_id(),
            name: name,
            file_type: FrontDirectoryContentType::Snippet,
            is_directory: false,
            level: level,
            showing: false,
        };

        visual_directory_component_manager.directory_front_to_directory_entry.insert(front_directory_content.id, directory_entry_uuid);

        return front_directory_content;
    }

    fn new_category_from_directory_entry(name: String, directory_entry_uuid: Uuid, directory_entry: &SnippetDirectoryCategory, level: u32, visual_directory_component_manager: &mut VisualDirectoryComponentManager, sequential_id_generator: &mut SequentialIdGenerator) -> FrontDirectoryContent {
        // if level == 1, which is root, then show, else, don't
        let mut showing = false;

        if level == 1 {
            showing = true;
        }

        let front_directory_content = FrontDirectoryContent {
            id: sequential_id_generator.get_id(),
            name: name,
            file_type: FrontDirectoryContentType::Snippet,
            is_directory: false,
            level: level,
            showing: showing
        };

        visual_directory_component_manager.directory_front_to_directory_entry.insert(front_directory_content.id, directory_entry_uuid);

        return front_directory_content;
    }


    /*
    /// create new front snippet content of type external snippet file container 
    pub fn new_snippet(directory_manager: &mut DirectoryManager, external_snippet_manager: &ExternalSnippetManager, sequential_id_generator: &mut SequentialIdGenerator, external_snippet_file_container: &SnippetDirectoryEntry, level: u32) -> Result<Self, String> {
        //TODO: ask why &str instead of String is not working
        //call front method on file container
        let front_external_snippet_content = match external_snippet_file_container.get_as_front_content(external_snippet_manager, sequential_id_generator, level) {
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

        //return external_snippet.get_as_front_content(visual_directory_component_manager, external_snippet_manager, sequential_id_generator, level);        
    }

    /// create new front snippet content of type category 
    pub fn new_category(visual_directory_component_manager: &mut VisualDirectoryComponentManager, sequential_id_generator: &mut SequentialIdGenerator, name: String,  level: u32) -> Self {
        return FrontDirectoryContent::new(
            sequential_id_generator.get_id(),
            name.clone(),
            FrontDirectoryContentType::Directory,
            true,
            level,
            false
        );
    }*/
}

