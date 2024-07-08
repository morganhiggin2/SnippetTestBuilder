use bimap::BiHashMap;
use serde::{Deserialize, Serialize};

use crate::{core_components::snippet_manager::SnippetManager, state_management::external_snippet_manager::ExternalSnippetManager, utils::sequential_id_generator::{SequentialIdGenerator, Uuid}};

use super::directory_manager::{DirectoryManager, SnippetDirectory, SnippetDirectoryCategory, SnippetDirectoryEntry, SnippetDirectorySnippet, SnippetDirectoryType};

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

    /// Get the directory as front elements
    /// These are displayed in descencing order, expandable and contractable based on parent
    /// If the directory were to get reloaded, we would need to reload the front directory
    pub fn get_directory_as_front(&mut self, snippet_directory: &SnippetDirectory, sequential_id_generator: &mut SequentialIdGenerator) -> Vec::<FrontDirectoryContent> {
        // Walk directory recursivly, keeping track of level, calling VisualDirectoryComponentManager::new_from_directory_entry
        let mut front_directory_content = Vec::<FrontDirectoryContent>::new();

        let root_directory_entry = match snippet_directory.get_root_directory_entry() {
            Some(some) => some,
            None => {
                //return Err("Root directory entry does not exist for snippet directory in get_directory_as_front method call, perhaps initializations has not occured");
                return front_directory_content;
            }
        };

        self.front_directory_walker_helper(root_directory_entry, 1, &mut front_directory_content, sequential_id_generator);

        return front_directory_content; 
    }

    fn front_directory_walker_helper(&mut self, current_directory_entry: &SnippetDirectoryEntry, level: u32, front_directory_content: &mut Vec::<FrontDirectoryContent>, sequential_id_generator: &mut SequentialIdGenerator) {
        let name = current_directory_entry.get_name();
        let uuid = current_directory_entry.get_uuid();

        match current_directory_entry.get_inner_as_ref() {
            SnippetDirectoryType::Category(category) => {
                // Create category as front
                let front_directory_entry = FrontDirectoryContent::new_category_from_directory_entry(name, uuid, category, level, self, sequential_id_generator);
                front_directory_content.push(front_directory_entry);

                // Call children of category directory entry
                for child_directory_entry in category.get_children() {
                    self.front_directory_walker_helper(child_directory_entry, level + 1, front_directory_content, sequential_id_generator);
                }
            },
            SnippetDirectoryType::Snippet(snippet) => {
                // Create snippet as front
                let front_directory_entry = FrontDirectoryContent::new_snippet_from_directory_entry(name, uuid, snippet, level, self, sequential_id_generator);
                front_directory_content.push(front_directory_entry);
            },
        };
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

