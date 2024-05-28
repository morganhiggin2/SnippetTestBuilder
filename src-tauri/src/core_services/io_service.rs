use std::{collections::HashMap, ffi::OsStr, fs::{self, DirEntry}, path::PathBuf};
use pyo3::callback::IntoPyCallbackOutput;
use serde::{Serialize, Deserialize};
use std::env;
use pathdiff;
use std::path::Path;

use crate::{core_components::snippet, state_management::external_snippet_manager::{ExternalSnippet, ExternalSnippetManager, IOContentType}, utils::sequential_id_generator::{SequentialIdGenerator, Uuid}};

use super::visual_directory_component_manager::{VisualDirectoryComponentManager, self};

pub struct DirectoryManager {
    relative_snippet_directory: String,
    //TODO remove pub
    pub snippet_structure: SnippetStructure,
    //visual front end components for directory contents
    pub visual_component_manager: VisualDirectoryComponentManager
}

//TODO remove pub
/// structure container for organization of snippets
pub struct SnippetStructure {
    //TODO remove all pubs
    pub root_category: Option<ExternalSnippetCategory>,
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
        let relative_snippet_directory = "data/snippets".to_string();

        //Only run if debug build, as the relative directory for the data will be different
        #[cfg(debug_assertions)]
        let relative_snippet_directory = "../data/snippets".to_string();

        return DirectoryManager {
            relative_snippet_directory: relative_snippet_directory,
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
    pub fn init(&mut self, external_snippet_manager: &mut ExternalSnippetManager, seq_id_generator: &mut SequentialIdGenerator) {
        //map snippet directory and get external snippets
        //TODO show error on panic, not just close the program
        self.snippet_structure.map_directory(external_snippet_manager, seq_id_generator, &self.relative_snippet_directory).unwrap();
    }


}

impl Default for SnippetStructure {
    fn default() -> Self {
        return SnippetStructure {
            root_category: None,
            categories: HashMap::new(),
            external_snippet_containers: HashMap::new()
        }
    }
}

impl SnippetStructure {
    /// reads snippet directory, reads all snippet files,
    /// and compiles all snippet category, file inforation,
    /// as well as assembles external snippets
    pub fn map_directory(&mut self, external_snippet_manager: &mut ExternalSnippetManager, seq_id_generator: &mut SequentialIdGenerator, relative_snippet_directory: &String) -> Result<(), String> {
        //get current working directory
        /*let current_working_directory = match env::current_dir() {
            Ok(result) => result.as_path().to_owned(),
            Err(e) => {
                return Err(e.to_string());
            }
        };    

        let snippets_directory = current_working_directory.join(relative_snippet_directory);

        //list of relative snippet script files
        let mut relative_snippet_directory_strings: Vec<String> = Vec::new();

        println!("{}", snippets_directory.to_owned().as_os_str().to_string_lossy());

        //read directory
        if snippets_directory.is_dir() {
            let dir_buf = match fs::read_dir(current_working_directory) {
                Ok(result) => result,
                Err(e) => {
                    return Err(e.to_string());
                }
            };


            //read directory contents in iterator
            for entry_result in dir_buf{

                let entry = match entry_result {
                    Ok(result) => result,
                    Err(e) => {
                        return Err(e.to_string())
                    }
                };

                let cur_path = entry.path();

                //filter out files with don't end in .py
                if let Some(file_extension) = cur_path.extension(){
                    if file_extension.eq(PYTHON_FILE_EXTENSION) {
                        
                    }
                }

                //get relative folder path as string
                let relative_string_path = match pathdiff::diff_paths(&cur_path, &snippets_directory) {
                    Some(result) => result.to_string_lossy().into_owned(),
                    None => {
                        return Err("directory of found snippet is not in snippets path, directory logic malfunction".to_string());
                    }
                };

                relative_snippet_directory_strings.push(relative_string_path);
            }
        }
        else {
            //create directory if it does not exist
            match fs::create_dir(snippets_directory) {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!("could not create snippet directory: {}", e.to_string()));
                }
            };
        }*/

        //get current working directory
        let current_working_directory = match env::current_dir() {
            Ok(result) => result.as_path().to_owned(),
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let snippets_directory = current_working_directory.join(relative_snippet_directory);

        //todo in snippet structure, use relative string path to get categories
        //list of snippets to create and their directories
        let mut snippet_factory_queue = Vec::<PathBuf>::new();

        //create root category
        let root_category = match self.root_category.as_ref() {
            Some(some) => some,
            None => {
                // create root category
                let root_category = ExternalSnippetCategory::new_root(seq_id_generator, "root".to_string(), 0, 1);
                // assign to self
                self.root_category = Some(root_category);

                //return reference
                //because we inserted some value above, we can safely unwrap here
                self.root_category.as_ref().unwrap()
            }
        };

        self.map_directory_walker_helper(&snippets_directory,root_category, external_snippet_manager, seq_id_generator, &mut snippet_factory_queue);

        return Ok(());
    }
    
    // Walk directory, and for each folder that is not a snippet, create a category
    fn map_directory_walker_helper(&mut self, current_path: &PathBuf, parent_category: &ExternalSnippetCategory, external_snippet_manager: &mut ExternalSnippetManager, seq_id_generator: &mut SequentialIdGenerator, snippet_factory_queue: &mut Vec<PathBuf>) -> Result<(), String> {
        // first check to see if there exists a .py file in the direcoty, regardless of the other contents
        // if there is a .py file in the directory, there this is a snippet
        // for each entry in the directory we are in 
        
        // get directory iterator
        let dir_iter = match fs::read_dir(current_path) {
            Ok(some) => some,
            Err(e) => {
                return Err(format!("Error in getting read dir for path {}: {}", current_path.as_os_str().to_string_lossy(), e.to_string()));
            }
        };

        // list of directories to iterate over
        // since the direcotry iterator incurrs a cost in iterating the path, we are going to store these results
        let mut directory_entries = Vec::<DirEntry>::new();
        
        for entry in dir_iter{
            // if the entry is load correctly
            if let Ok(dir_entry) = entry {
                // call directory walker on directory to recurrsivly dive into each folder
                if current_path.is_dir() {
                    directory_entries.push(dir_entry);
                }
                // it is a file
                else {
                    //check if this is a .py file
                    if let Some(file_extension) = current_path.extension(){
                        // if this is a .py file, this is a snippet
                        if file_extension.eq(OsStr::new("py")) {
                            //create unitilized snippet, then to be initialized after directory walking
                            snippet_factory_queue.push(current_path.clone());

                            // end the directory search
                            // any other directories in this directory can be considered for the snippet, as a snippet cannot exist inside a snippet
                            return Ok(());
                        }
                    }
                    else {
                        // misc files deal with here
                        // for now, this is going to be ignored
                    }
                }
            }
            else {
                //TODO log error in getting dir entry, maybe some permission issue or what not
            }
        }

        // at this point, a snippet was not found in the directory, so this is considered to be a category

        // get name of the directory
        let category_name = match current_path.file_name() {
            Some(some) => some,
            None => {
                return Err(format!("Could not get name of directory at path {}", current_path.to_string_lossy()));
            }
        };

        let category_name = match category_name.to_str() {
            Some(some) => some,
            None => {
                return Err(format!("Could not get string of name of directory at path {}", current_path.to_string_lossy()));
            }
        };
        
        // create category snippet
        // because we don't know the statistics ahead of time, we are going to use a default value of 1 for both
        let category = ExternalSnippetCategory::new_child(seq_id_generator, category_name.to_string(), 1, 1, parent_category.get_uuid()); 

        // recurrisvly search subdirectories
        for dir_entry in directory_entries {
            self.map_directory_walker_helper(&dir_entry.path(), &category, external_snippet_manager, seq_id_generator, snippet_factory_queue)?;
        }

        // insert category into self
        self.categories.insert(category.get_uuid(), category); 


       //if we are a directory
            // if this directory has a .py file, and name matches the folder name, it is a snippet, do not recurse further into it
            // we leave the freedom to the snippet creator to add their own files to the snippet

            // if does not contain matching .py file or sub directory, throw an error 

            // if a directory (so no .py which matches the directory name), go into it, call map_directory_walker_helper
        //else, 
            
            
        //if directory, return true, else, false

        return Ok(());
    }

    pub fn file_structure_to_front_snippet_contents(&self, visual_directory_component_manager: &mut VisualDirectoryComponentManager, seq_id_generator: &mut SequentialIdGenerator, external_snippet_manager: &mut ExternalSnippetManager) -> Vec<FrontExternalSnippetContent> {
        let mut front_snippet_contents: Vec<FrontExternalSnippetContent> = Vec::with_capacity(self.external_snippet_containers.len());

        //recursivly iterate though structure with helper function, reference to vec to add front file contents to
        let root_category = match self.root_category.as_ref() {
            Some(some) => some,
            None => {
                // no root category was created, so exit
                return front_snippet_contents;
            }
        };
        
        //get external snippet subcategory
        let external_snippet_category = self.find_category(&root_category.get_uuid()).unwrap();

        //create front snippet content
        let front_snippet_content = FrontExternalSnippetContent::new_category(visual_directory_component_manager, seq_id_generator, external_snippet_category.get_name(), 0);

        //add to front snippet contents
        front_snippet_contents.push(front_snippet_content);

        self.file_structure_to_front_snippet_contents_helper(visual_directory_component_manager, seq_id_generator, external_snippet_manager, &mut front_snippet_contents, external_snippet_category, 1);
        //SnippetStructure::file_structure_to_front_snippet_contents_helper(seq_id_generator, &mut front_snippet_contents, cat, 0);

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

    /// get list of snippets and their respective relative locations 
    pub fn get_snippets_and_locations(&self) -> Result<Vec<String>, &'static str> {
        todo!();
    }
}

impl ExternalSnippetCategory {
    pub fn new_root(seq_id_generator: &mut SequentialIdGenerator, name: String, num_snippets: usize, num_categories: usize) -> Self {
        return ExternalSnippetCategory {
            uuid: seq_id_generator.get_id(),
            name: name,
            parent_category_uuid: None,
            child_snippet_uuids: Vec::with_capacity(num_snippets),
            child_category_uuids: Vec::with_capacity(num_categories)
        };
    }
   
    fn new_child(seq_id_generator: &mut SequentialIdGenerator, name: String, num_snippets: usize, num_categories: usize, parent_category_uuid: Uuid) -> Self {
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
