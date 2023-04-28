use std::{fs, collections::HashMap};
use serde::{Serialize, Deserialize};

use crate::{utils::sequential_id_generator::{Uuid, SequentialIdGenerator}, state_management::{external_snippet_manager::{ExternalSnippetManager, IOContentType, ExternalSnippet}}};

use super::visual_directory_component_manager::{VisualDirectoryComponentManager, self};

pub struct DirectoryManager {
    //TODO remove pub
    pub snippet_structure: SnippetStructure,
    //visual front end components
    pub visual_component_manager: VisualDirectoryComponentManager
}

//TODO remove pub
/// structure container for organization of snippets
pub struct SnippetStructure {
    //TODO remove all pubs
    pub root_categories: Vec<Uuid>,
    pub categories: HashMap<Uuid, ExternalSnippetCategory>,
    pub external_snippet_containers: HashMap<Uuid, ExternalSnippetFileContainer>
}

//TODO remove pub
pub struct ExternalSnippetCategory {
    uuid: Uuid,
    name: String,
    parent_category_uuid: Option<Uuid>,
    //TODO remove pub
    pub child_snippet_uuids: Vec<Uuid>,
    child_category_uuids: Vec<Uuid> 
}

//TODO remove pub
pub struct ExternalSnippetFileContainer {
    uuid: Uuid,
    external_snippet_uuid: Uuid,
    parent_category_uuid: Uuid
}

//struct for the josn serialization
#[derive(Serialize, Deserialize)]
pub struct FrontExternalSnippetContent {
    id: Uuid,
    name: String,
    file_type: FrontExternalSnippetContentType,
    is_directory: bool,
    level: u32,
    showing: bool,
}

#[derive(Serialize, Deserialize)]
pub enum FrontExternalSnippetContentType {
    Directory,
    Snippet
}

impl Default for DirectoryManager {
    fn default() -> Self {
        return DirectoryManager {
            snippet_structure: SnippetStructure::default(),
            visual_component_manager: VisualDirectoryComponentManager::default()
        }
    }
}

impl DirectoryManager {
    

    /*pub fn map_directory(directory : & str) -> Result<Self, &str>{
        //count directory
        let paths = match fs::read_dir(&directory) {
            Ok(result) => result,
            Error => return Result::Err("directory could not be red") 
        };

        let num_files = paths.count();
        
        //alloc vector of that size
        let file_structure : DirectoryManager = DirectoryManager {
            file_list: Vec::with_capacity(num_files)
        };

        //read directory, getting file names and directory path, type, etc
        /*
        for path in paths {
           file_structure.file_list.push(path); 
        }*/

        return Ok(file_structure);
    }*/

    /// initalize file directory system
    /// and related subsystems
    pub fn init(&mut self, external_snippet_manager: &mut ExternalSnippetManager) {
        //map snippet directory and get external snippets
        self.snippet_structure.map_directory(external_snippet_manager);
    }


}

impl Default for SnippetStructure {
    fn default() -> Self {
        return SnippetStructure {
            root_categories: Vec::new(),
            categories: HashMap::new(),
            external_snippet_containers: HashMap::new()
        }
    }
}

impl SnippetStructure {
    /// reads snippet directory, reads all snippet files,
    /// and compiles all snippet category, file inforation,
    /// as well as assembles external snippets
    pub fn map_directory(&mut self, external_snippet_manager: &mut ExternalSnippetManager) {

    }

    pub fn file_structure_to_front_snippet_contents(&self, visual_directory_component_manager: &mut VisualDirectoryComponentManager, seq_id_generator: &mut SequentialIdGenerator, external_snippet_manager: &mut ExternalSnippetManager) -> Vec<FrontExternalSnippetContent> {
        let mut front_snippet_contents: Vec<FrontExternalSnippetContent> = Vec::with_capacity(self.external_snippet_containers.len());

        //recursivly iterate though structure with helper function, reference to vec to add front file contents to
        for cat_uuid in self.root_categories.iter() {
            //get external snippet subcategory
            let external_snippet_category = self.find_category(&cat_uuid).unwrap();

            //create front snippet content
            let front_snippet_content = FrontExternalSnippetContent::new_category(visual_directory_component_manager, seq_id_generator, external_snippet_category.get_name(), 0);

            //add to front snippet contents
            front_snippet_contents.push(front_snippet_content);

            self.file_structure_to_front_snippet_contents_helper(visual_directory_component_manager, seq_id_generator, external_snippet_manager, &mut front_snippet_contents, external_snippet_category, 1);
            //SnippetStructure::file_structure_to_front_snippet_contents_helper(seq_id_generator, &mut front_snippet_contents, cat, 0);
        }

        return front_snippet_contents;
    }

    /// helper function to snippet_structure_to_front_snippet_contents
    /// recursivly goes though snippet structure
    fn file_structure_to_front_snippet_contents_helper(&self, visual_directory_component_manager: &mut VisualDirectoryComponentManager, seq_id_generator: &mut SequentialIdGenerator, external_snippet_manager: &mut ExternalSnippetManager, front_snippet_contents: &mut Vec<FrontExternalSnippetContent>, external_snippet_category: &ExternalSnippetCategory, level: u32) {
        //add external snippets
        for ext_snip_uuid in external_snippet_category.child_snippet_uuids.iter() {
            //find external snippet file container
            let external_snippet_container = self.find_external_snippet_container(ext_snip_uuid).unwrap(); 

            //create front snippet content
            //can safely unwrap since we created the external snippet before this method call
            //nothing else could change the existance or properties of it before
            let front_snippet_content = FrontExternalSnippetContent::new_snippet(visual_directory_component_manager, external_snippet_manager, &self, seq_id_generator, &external_snippet_container, level).unwrap();

            //add to front snippet contents
            front_snippet_contents.push(front_snippet_content)
        }

        //go into external snippet categories
        for cat_uuid in external_snippet_category.child_category_uuids.iter() {
            //get external snippet subcategory
            let external_snippet_category = self.find_category(&cat_uuid).unwrap();

            //create front snippet content
            let front_snippet_content = FrontExternalSnippetContent::new_category(visual_directory_component_manager, seq_id_generator, external_snippet_category.get_name(), 0);
            //add to front snippet contents
            front_snippet_contents.push(front_snippet_content);

            //call helper to go into category recurrsivly
            self.file_structure_to_front_snippet_contents_helper(visual_directory_component_manager, seq_id_generator, external_snippet_manager, front_snippet_contents, external_snippet_category, level + 1);
        }
    }
    
    /// find category given uuid
    pub fn find_category(&self, uuid: &Uuid) -> Option<&ExternalSnippetCategory> {
        return self.categories.get(uuid); 
    }

    /// find external snippet container given uuid
    pub fn find_external_snippet_container(&self, uuid: &Uuid) -> Option<&ExternalSnippetFileContainer> {
        return self.external_snippet_containers.get(uuid);
    }

    /// find category given uuid
    pub fn find_category_mut(&mut self, uuid: &Uuid) -> Option<&mut ExternalSnippetCategory> {
        return self.categories.get_mut(uuid); 
    }

    /// find external snippet container given uuid
    pub fn find_external_snippet_container_mut(&mut self, uuid: &Uuid) -> Option<&mut ExternalSnippetFileContainer> {
        return self.external_snippet_containers.get_mut(uuid);
    }
}

impl ExternalSnippetCategory {
    pub fn new_parent(seq_id_generator: &mut SequentialIdGenerator, name: String, num_snippets: usize, num_categories: usize) -> Self {
        return ExternalSnippetCategory {
            uuid: seq_id_generator.get_id(),
            name: name,
            parent_category_uuid: None,
            child_snippet_uuids: Vec::with_capacity(num_snippets),
            child_category_uuids: Vec::with_capacity(num_categories)
        };
    }
   
    fn new_sub(seq_id_generator: &mut SequentialIdGenerator, name: String, num_snippets: usize, num_categories: usize, parent_category_uuid: Uuid) -> Self {
        return ExternalSnippetCategory {
            uuid: seq_id_generator.get_id(),
            name: name,
            parent_category_uuid: Some(parent_category_uuid),
            child_snippet_uuids: Vec::with_capacity(num_snippets),
            child_category_uuids: Vec::with_capacity(num_categories)
        };
    }
    
    //TODO remove pub
    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }
    
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl ExternalSnippetFileContainer {
    //TODO remove new
    pub fn new(seq_id_generator: &mut SequentialIdGenerator, external_snippet_uuid: Uuid, parent_category_uuid: Uuid) -> Self {
        return ExternalSnippetFileContainer {
            uuid: seq_id_generator.get_id(),
            external_snippet_uuid: external_snippet_uuid,
            parent_category_uuid: parent_category_uuid
        };
    }

    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }

    pub fn get_external_snippet_uuid(&self) -> Uuid {
        return self.external_snippet_uuid;
    }

    pub fn get_as_front_content(&self, external_snippet_manager: &ExternalSnippetManager, seq_id_generator: &mut SequentialIdGenerator, level: u32) -> Result<FrontExternalSnippetContent, &str>{
        //get external snippet 
        let external_snippet = match external_snippet_manager.find_external_snippet(self.get_external_snippet_uuid()) {
            Ok(result) => result,
            Err(e) => {
                return Err("could not find external snippet for uuid in external snippet file container");
            }
        };

        let content = FrontExternalSnippetContent::new(
            seq_id_generator.get_id(),
            external_snippet.get_name(),
            self.get_uuid(),
            FrontExternalSnippetContentType::Snippet,
            false,
            level,
            false,
        );
        
        return Ok(content);
    }
}

impl FrontExternalSnippetContent {
    pub fn new(uuid: Uuid, name: String, internal_id: Uuid, file_type: FrontExternalSnippetContentType, is_directory: bool, level: u32, showing: bool) -> Self {
        let front_content = FrontExternalSnippetContent {
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
    fn new_snippet(visual_directory_component_manager: &mut VisualDirectoryComponentManager, external_snippet_manager: &ExternalSnippetManager, snippet_structure: &SnippetStructure, seq_id_generator: &mut SequentialIdGenerator, external_snippet_file_container: &ExternalSnippetFileContainer, level: u32) -> Result<Self, String> {
        //TODO: ask why &str instead of String is not working
        //call front method on file container
        let front_external_snippet_content = match external_snippet_file_container.get_as_front_content(external_snippet_manager, seq_id_generator, level) {
            Ok(result) => result,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        //add front content to visual component manager
        visual_directory_component_manager.put_snippet_file_container_uuid(front_external_snippet_content.id, external_snippet_file_container.get_uuid()); 

        return Ok(front_external_snippet_content);

        //TODO delete these comments
        //find external snippet
        //let external_snippet = external_snippet_manager.find_external_snippet(external_snippet_container.get_external_snippet_uuid()).unwrap();

        //return external_snippet.get_as_front_content(visual_directory_component_manager, external_snippet_manager, seq_id_generator, level);        
    }

    /// create new front snippet content of type category 
    fn new_category(visual_directory_component_manager: &mut VisualDirectoryComponentManager, seq_id_generator: &mut SequentialIdGenerator, name: String,  level: u32) -> Self {
        return FrontExternalSnippetContent::new(
            seq_id_generator.get_id(),
            name.clone(),
            0,
            FrontExternalSnippetContentType::Directory,
            true,
            level,
            false,
        );
    }
}