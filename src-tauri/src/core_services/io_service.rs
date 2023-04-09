use std::{fs, collections::HashMap};
use serde::{Serialize, Deserialize};

use crate::{utils::sequential_id_generator::{Uuid, SequentialIdGenerator}, state_management::external_snippet_manager::{self, ExternalSnippetManager}};

pub struct DirectoryManager {
    //TODO remove pub
    pub snippet_structure: SnippetStructure
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
pub struct FrontSnippetContent {
    id: Uuid,
    name: String,
    is_directory: bool,
    level: u32,
    showing: bool,
}

impl Default for DirectoryManager {
    fn default() -> Self {
        return DirectoryManager {
            snippet_structure: SnippetStructure::default()
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

    pub fn file_structure_to_front_snippet_contents(&self, seq_id_generator: &mut SequentialIdGenerator, external_snippet_manager: &mut ExternalSnippetManager) -> Vec<FrontSnippetContent> {
        let mut front_snippet_contents: Vec<FrontSnippetContent> = Vec::with_capacity(self.external_snippet_containers.len());

        //recursivly iterate though structure with helper function, reference to vec to add front file contents to
        for cat_uuid in self.root_categories.iter() {
            //get external snippet subcategory
            let external_snippet_category = self.find_category(&cat_uuid).unwrap();

            //create front snippet content
            let front_snippet_content = FrontSnippetContent::new_category(seq_id_generator, external_snippet_category.get_name(), 0);

            //add to front snippet contents
            front_snippet_contents.push(front_snippet_content);

            self.file_structure_to_front_snippet_contents_helper(seq_id_generator, external_snippet_manager, &mut front_snippet_contents, external_snippet_category, 1);
            //SnippetStructure::file_structure_to_front_snippet_contents_helper(seq_id_generator, &mut front_snippet_contents, cat, 0);
        }

        return front_snippet_contents;
    }

    /// helper function to snippet_structure_to_front_snippet_contents
    /// recursivly goes though snippet structure
    fn file_structure_to_front_snippet_contents_helper(&self, seq_id_generator: &mut SequentialIdGenerator, external_snippet_manager: &mut ExternalSnippetManager, front_snippet_contents: &mut Vec<FrontSnippetContent>, external_snippet_category: &ExternalSnippetCategory, level: u32) {
        //add external snippets
        for ext_snip_uuid in external_snippet_category.child_snippet_uuids.iter() {
            //find external snippet file container
            let external_snippet_container = self.find_external_snippet_container(ext_snip_uuid).unwrap(); 

            //find external snippet
            let external_snippet = external_snippet_manager.find_external_snippet(external_snippet_container.get_external_snippet_uuid()).unwrap();

            //create front snippet content
            let front_snippet_content = FrontSnippetContent::new_snippet(seq_id_generator, external_snippet.get_name(), level);

            //add to front snippet contents
            front_snippet_contents.push(front_snippet_content)
        }

        //go into external snippet categories
        for cat_uuid in external_snippet_category.child_category_uuids.iter() {
            //get external snippet subcategory
            let external_snippet_category = self.find_category(&cat_uuid).unwrap();

            //create front snippet content
            let front_snippet_content = FrontSnippetContent::new_category(seq_id_generator, external_snippet_category.get_name(), 0);
            //add to front snippet contents
            front_snippet_contents.push(front_snippet_content);

            //call helper to go into category recurrsivly
            self.file_structure_to_front_snippet_contents_helper(seq_id_generator, external_snippet_manager, front_snippet_contents, external_snippet_category, level + 1);
        }
    }
    
    /// find category given uuid
    fn find_category(&self, uuid: &Uuid) -> Option<&ExternalSnippetCategory> {
        return self.categories.get(uuid); 
    }

    /// find external snippet container given uuid
    fn find_external_snippet_container(&self, uuid: &Uuid) -> Option<&ExternalSnippetFileContainer> {
        return self.external_snippet_containers.get(uuid);
    }

    /// find category given uuid
    fn find_category_mut(&mut self, uuid: &Uuid) -> Option<&mut ExternalSnippetCategory> {
        return self.categories.get_mut(uuid); 
    }

    /// find external snippet container given uuid
    fn find_external_snippet_container_mut(&mut self, uuid: &Uuid) -> Option<&mut ExternalSnippetFileContainer> {
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

    //TODO remove pub
    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }

    fn get_external_snippet_uuid(&self) -> Uuid {
        return self.external_snippet_uuid;
    }
}

impl FrontSnippetContent {
    /// create new front snippet content of type snippet 
    fn new_snippet(seq_id_generator: &mut SequentialIdGenerator, name: String, level: u32) -> Self {
        return FrontSnippetContent {
            id: seq_id_generator.get_id(),
            name: name.clone(),
            is_directory: false,
            level: level,
            showing: false
        };
    }

    /// create new front snippet content of type category 
    fn new_category(seq_id_generator: &mut SequentialIdGenerator, name: String, level: u32) -> Self {
        return FrontSnippetContent {
            id: seq_id_generator.get_id(),
            name: name.clone(),
            is_directory: true,
            level: level,
            showing: false
        };
    }
}