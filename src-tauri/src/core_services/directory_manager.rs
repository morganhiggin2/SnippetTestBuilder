use core::fmt;
use std::{borrow::Borrow, collections::HashMap, ffi::OsStr, fs::{self, DirEntry}, io::{self, Empty}, path::{Display, PathBuf}, rc::Rc, sync::Arc};
use enum_as_inner::EnumAsInner;
use serde::{Serialize, Deserialize};
use std::env;
use pathdiff;
use std::path::Path;

use crate::{core_components::snippet_manager, state_management::external_snippet_manager::{ExternalSnippet, ExternalSnippetCategory, ExternalSnippetManager}, utils::sequential_id_generator::{self, SequentialIdGenerator, Uuid}};

use super::{visual_directory_component_manager::{FrontDirectoryContent, FrontDirectoryContentType}, visual_directory_component_manager::{self, VisualDirectoryComponentManager}};

// This here is not ui related
pub struct DirectoryManager {
    relative_snippet_directory: String,
    //TODO remove pub
    pub snippet_directory: SnippetDirectory,
    //visual front end components for directory contents
    pub visual_component_manager: VisualDirectoryComponentManager
}

// Actually going to be a by-reference instead of by-uuid-lookup implementation
// A snippet directory has a root snippet directory entry
// a snippet directory entry is either a snippet directory snippet or a snippet directory category
// a snippet directory category can be a parent to other snippet directory entries
pub struct SnippetDirectory {
    root: Option<SnippetDirectoryEntry>
}

pub struct SnippetDirectoryEntry {
    name: String,
    uuid: Uuid,
    content: SnippetDirectoryType,
    path: PathBuf
}

#[derive(EnumAsInner)]
pub enum SnippetDirectoryType {
    Category(SnippetDirectoryCategory),
    Snippet(SnippetDirectorySnippet)
}

pub struct SnippetDirectorySnippet {

}

pub struct SnippetDirectoryCategory {
    children: Vec<SnippetDirectoryEntry>
}

impl DirectoryManager {
    // Initialize the directory manager
    pub fn initialize(&mut self, sequential_id_generator: &mut SequentialIdGenerator) -> Result<(), String> {
        // First create the snippet directory
        self.snippet_directory.initialize(sequential_id_generator)?;

        return Ok(());
    }
}

impl Default for SnippetDirectory {
    fn default() -> Self {
        return SnippetDirectory {
            root: None 
        };
    }
}

impl SnippetDirectory {
    /// Initialize the snippet directory
    pub fn initialize(&mut self, sequential_id_generator: &mut SequentialIdGenerator) -> Result<(), String> {
        self.scan_and_map_directory(&"../data/snippets/main/".to_string(), sequential_id_generator)?;
        
        return Ok(());
    }
     ///reads snippet directory, reads all snippet files,
    /// and compiles all snippet category, file inforation,
    /// as well as assembles external snippets, existing snippets will not be overriden
    /// and new snippets will be inserted
    fn scan_and_map_directory(&mut self, relative_snippet_directory: &String, sequential_id_generator: &mut SequentialIdGenerator) -> Result<(), String> {
        //get current working directory
        let current_working_directory = match env::current_dir() {
            Ok(result) => result.as_path().to_owned(),
            Err(e) => {
                return Err(e.to_string());
            }
        };
        //get snippet directory
        let snippets_directory = current_working_directory.join(relative_snippet_directory);

        // if root category does not exist, override it 
        match self.root {
            Some(_) => (), 
            None => {
                // create root category
                let root_category = SnippetDirectoryEntry::new_category("main".to_owned(), snippets_directory.to_owned(), sequential_id_generator); 

                self.root = Some(root_category);
            }
        };

        // becausse we know the root is of type Some, we can safely unwrap
        let root = self.root.as_mut().unwrap();
        let root = root.get_as_category().unwrap();

        // walk directory recurrsivly
        SnippetDirectory::directory_walker(root, &snippets_directory, sequential_id_generator)?;
      
        return Ok(());
    }
    // the next step would to call initialize snippet to create snippets on every snippet file container
    // this will create the snippets, and populate the mapping from external snippet front container to external snippets

    /// walk directory method, that looks for snippets, when it reaches a <snippet_name>.py file in the current directory,
    ///   it calls another helper to walk that dirctory for all .py files, list of .py file pathbufs, and 
    ///   by the end of this we have a full directory structure. 
    fn directory_walker(parent_directory_category: &mut SnippetDirectoryCategory, current_path: &Path, sequential_id_generator: &mut SequentialIdGenerator) -> Result<(), String> {
        // Change this to match other logic for seeing it is a snippet, possibly a is_snippet method instead of is_dir

        // Get if snippet directory
        let is_snippet_directory = SnippetDirectory::is_directory_snippet(current_path)?;

        let dir_name = match current_path.file_stem() {
            Some(some) => some,
            None => &OsStr::new("")
        };

        let dir_name = match dir_name.to_str() {
            Some(some) => some,
            None => ""
        };

        if is_snippet_directory {

            // create snippet type, add as child 
            let snippet_entry = SnippetDirectoryEntry::new_snippet(dir_name.to_owned(), current_path.to_owned(), sequential_id_generator);

            // add as child
            parent_directory_category.add_child(snippet_entry);

        }
        else if current_path.is_dir() {
            // create category
            let mut snippet_entry = SnippetDirectoryEntry::new_category(dir_name.to_owned(), current_path.to_owned(), sequential_id_generator);

            // get snippet category type
            let snippet_category = snippet_entry.get_as_category()?; 

            let dir_entries = match fs::read_dir(current_path) {
                Ok(some) => some,
                Err(e) => {
                    return Err(format!("Could not get directory entry for a given directory path in directory_walker call: {}", e.to_string()));
                }
            };
        
            // walk sub 
            for directory_entry in dir_entries {
                let entry = match directory_entry {
                    Ok(some) => some,
                    Err(e) => {
                        return Err(format!("Could not get entry from directory entry in directory_walker: {}", e.to_string()));
                    }
                };
                let path = entry.path();

                if path.is_dir() {
                    SnippetDirectory::directory_walker(snippet_category, current_path, sequential_id_generator)?;
                }
            }

            // add as child to parent category
            parent_directory_category.add_child(snippet_entry);
        }

        //else this is a random file or something, ignore

        return Ok(());
    }

    fn is_directory_snippet(dir_path: &Path) -> Result<bool, String> {
        if dir_path.is_dir() {
            let dir_entries = match fs::read_dir(dir_path) {
                Ok(some) => some,
                Err(e) => {
                    return Err(format!("Could not get directory entry for a given directory path in is_directory_snippet call: {}", e.to_string()));
                }
            };

            // walk dir
            for directory_entry in dir_entries {
                let entry = match directory_entry {
                    Ok(some) => some,
                    Err(e) => {
                        return Err(format!("Could not get entry from directory entry in is_directory_snippet: {}", e.to_string()));
                    }
                };

                // check if this is a . file
                if let Some(file_extension) = entry.path().extension(){
                    // get file name
                    let file_name = entry.file_name();

                    // println!(file_extension);
                    // if this is a app.py file, this is a snippet
                    if file_extension.eq(OsStr::new("py")) && file_name.eq(OsStr::new("app")) {
                        //create unitilized snippet, then to be initialized after directory walking
                        return Ok(true);
                    }    
                }
            }
        }

        return Ok(false);
    }
    
    /*
    /// Walk directory, and for each folder that is not a snippet, create a category
    fn map_directory_walker_helper(&mut self, current_path: &PathBuf, parent_category: &mut ExternalSnippetCategory, sequential_id_generator: &mut SequentialIdGenerator) -> Result<(), String> {
        // first check to see if there exists a . file in the direcoty, regardless of the other contents
        // if there is a . file in the directory, there this is a snippet
        // for each entry in the directory we are in 
        
        // get directory iterator
        let dir_iter = match fs::read_dir(current_path) {
            Ok(some) => some,
            Err(e) => {
                return Err(format!("Error in getting read dir for path {}: {}", current_path.as_os_str().to_string_lossy(), e.to_string()));
            }
        };

        // list of directories to iterate over
        // since the direcotry iterator incurs a cost in iterating the path, we are going to store these results
        // tempoarary variable
        let mut directory_entries = Vec::<DirEntry>::new();
        
        for entry in dir_iter{
            // if the entry is load correctly
            if let Ok(dir_entry) = entry {
                // call directory walker on directory to recurrsivly dive into each folder
                if dir_entry.path().is_dir() {
                    directory_entries.push(dir_entry);
                }
                // it is a file
                else {
                    
                    //check if this is a . file
                    if let Some(file_extension) = dir_entry.path().extension(){
                        // get file name
                        let file_name = dir_entry.file_name();

                        // println!(file_extension);
                        // if this is a app.py file, this is a snippet
                        if file_extension.eq(OsStr::new("py")) && file_name.eq(OsStr::new("app")) {
                            //create unitilized snippet, then to be initialized after directory walking
                            snippet_factory_queue.push(dir_entry.path());

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

        return Ok(());
    }*/

    pub fn get_root_directory_entry(&self) -> Option<&SnippetDirectoryEntry> {
        return self.root.as_ref();
    }
}

impl SnippetDirectoryEntry {
    pub fn new_category(name: String, path: PathBuf, sequential_id_generator: &mut SequentialIdGenerator) -> Self {
        return SnippetDirectoryEntry {
            name: name,
            uuid: sequential_id_generator.get_id(),
            content: SnippetDirectoryType::Category(SnippetDirectoryCategory::new()),
            path: path
        }
    }

    pub fn new_snippet(name: String, path: PathBuf, sequential_id_generator: &mut SequentialIdGenerator) -> Self {
        return SnippetDirectoryEntry {
            name: name,
            uuid: sequential_id_generator.get_id(),
            content: SnippetDirectoryType::Snippet(SnippetDirectorySnippet::new()),
            path: path
        }
    }

    pub fn get_as_category(&mut self) -> Result::<&mut SnippetDirectoryCategory, String> {
        match &mut self.content {
            SnippetDirectoryType::Category(some) => {
                return Ok(some);
            }
            SnippetDirectoryType::Snippet(_) => {
                return Err("Directory Entry is not a category, in call to get_as_category".to_string());
            }
        }
    }
    
    pub fn get_as_snippet(&mut self) -> Result::<&mut SnippetDirectorySnippet, String> {
        match &mut self.content {
            SnippetDirectoryType::Category(_) => {
                return Err("Directory Entry is not a snippet, in call to get_as_snippet".to_string());
            }
            SnippetDirectoryType::Snippet(some) => {
                return Ok(some);
            }
        }
    }

    pub fn get_inner_as_ref(&self) -> &SnippetDirectoryType {
        return &self.content;
    }

    pub fn get_name(&self) -> String {
        return self.name.to_owned();
    }

    pub fn get_path(&self) -> PathBuf {
        return self.path.to_owned();
    }
}

impl SnippetDirectoryCategory {
    fn new() -> Self {
        return SnippetDirectoryCategory{
            children: Vec::<SnippetDirectoryEntry>::new()
        };
    }

    pub fn add_child(&mut self, child: SnippetDirectoryEntry) {
        self.children.push(child);
    }

    pub fn get_children(&self) -> &Vec::<SnippetDirectoryEntry> {
        return &self.children;
    }
}

impl SnippetDirectorySnippet {
    fn new() -> Self {
        return SnippetDirectorySnippet {  }; 
    }
}










/* 
//TODO remove pub
pub struct SnippetStructure {
    //TODO remove all pubs
    pub root_category: Option<ExternalSnippetCategory>,
    // map of uuid of the external categories to it's external category instance
    pub categories: HashMap<Uuid, ExternalSnippetCategory>,
    // map of uuid of external snippet file containers to the external snippet file container
    pub external_snippet_containers: HashMap<Uuid, ExternalSnippetFileContainer>
    //TODO snippet mapper that converts beteen external snippet file container to external snippet
    //  ..this mapping will be in the external snippet manager
}

//TODO remove pub
pub struct ExternalSnippetFileContainer {
    uuid: Uuid,
    parent_category_uuid: Uuid,
    python_files: Vec<PathBuf>
}


impl Default for DirectoryManager {
    fn default() -> Self {
        let relative_snippet_directory = "../data/snippets".to_string();

        //Only run if debug build, as the relative directory for the data will be different
        //#[cfg(debug_assertions)]
        //relative_snippet_directory = "../data/snippets".to_string();

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
    pub fn init(&mut self, external_snippet_manager: &mut ExternalSnippetManager, sequential_id_generator: &mut SequentialIdGenerator) {
        //map snippet directory and get external snippets
        //TODO show error on panic, not just close the program
        //self.snippet_structure.map_directory(external_snippet_manager, sequential_id_generator, &self.relative_snippet_directory).unwrap();
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
    /*
    /// reads snippet directory, reads all snippet files,
    /// and compiles all snippet category, file inforation,
    /// as well as assembles external snippets
    pub fn map_directory(&mut self, external_snippet_manager: &mut ExternalSnippetManager, sequential_id_generator: &mut SequentialIdGenerator, relative_snippet_directory: &String) -> Result<(), String> {
        //get current working directory
        let current_working_directory = match env::current_dir() {
            Ok(result) => result.as_path().to_owned(),
            Err(e) => {
                return Err(e.to_string());
            }
        };
        //get snippet directory
        let snippets_directory = current_working_directory.join(relative_snippet_directory);
        
        // if the root category already exists, we overrite it
        match self.root_category {
            Some(_) => {
                self.root_category = None;
                //TODO feature enhancement:
                // when calling this method, get rid of categories that are gone 
                // and when an existing one is attempted to be added in the map_directory_walker_helper method, it does not override an existing, 
                // same category with the same one (cause it will have a new uuid and mess up existing references) 
                return Err("Cannot call this method when there already exists a snippet strcture".to_string());
            },
            None => ()
        };
        // create root category
        let mut root_category = ExternalSnippetCategory::new_root(sequential_id_generator, "root".to_string(), 0, 1);

        // walk directory recurrsivly
        self.map_directory_walker_helper(&snippets_directory,&mut root_category, external_snippet_manager, sequential_id_generator)?;

        // set root category
        self.root_category = Some(root_category); 

        // create snippets
        self.create_snippets(&mut snippet_factory_queue)?;
      
        return Ok(());
    }
    
    // New walk directory method, that looks for snippets, when it reaches a .app file in the current directory
    //   calls another helper to walk that dirctory for all .py files, list of .py file pathbufs
    //   creates external snippet file container with this list
    // by the end of this we have a full directory structure. 
    // the next step would to call initialize snippet to create snippets on every snippet file container
    // this will create the snippets, and populate the mapping from external snippet front container to external snippets
    
    /// Walk directory, and for each folder that is not a snippet, create a category
    fn map_directory_walker_helper(&mut self, current_path: &PathBuf, parent_category: &mut ExternalSnippetCategory, external_snippet_manager: &mut ExternalSnippetManager, sequential_id_generator: &mut SequentialIdGenerator, snippet_factory_queue: &mut Vec<PathBuf>) -> Result<(), String> {
        // first check to see if there exists a . file in the direcoty, regardless of the other contents
        // if there is a . file in the directory, there this is a snippet
        // for each entry in the directory we are in 
        
        // get directory iterator
        let dir_iter = match fs::read_dir(current_path) {
            Ok(some) => some,
            Err(e) => {
                return Err(format!("Error in getting read dir for path {}: {}", current_path.as_os_str().to_string_lossy(), e.to_string()));
            }
        };

        // list of directories to iterate over
        // since the direcotry iterator incurs a cost in iterating the path, we are going to store these results
        // tempoarary variable
        let mut directory_entries = Vec::<DirEntry>::new();
        
        for entry in dir_iter{
            // if the entry is load correctly
            if let Ok(dir_entry) = entry {
                // call directory walker on directory to recurrsivly dive into each folder
                if dir_entry.path().is_dir() {
                    directory_entries.push(dir_entry);
                }
                // it is a file
                else {
                    
                    //check if this is a . file
                    if let Some(file_extension) = dir_entry.path().extension(){
                        // get file name
                        let file_name = dir_entry.file_name();

                        // println!(file_extension);
                        // if this is a app.py file, this is a snippet
                        if file_extension.eq(OsStr::new("py")) && file_name.eq(OsStr::new("app")) {
                            //create unitilized snippet, then to be initialized after directory walking
                            snippet_factory_queue.push(dir_entry.path());

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
        let mut category = ExternalSnippetCategory::new_child(sequential_id_generator, category_name.to_string(), 1, 1, parent_category.get_uuid()); 
        // add uuid link to parent category
        parent_category.add_child_category(&category);

        // recurrisvly search subdirectories
        for dir_entry in directory_entries {
            self.map_directory_walker_helper(&dir_entry.path(), &mut category, external_snippet_manager, sequential_id_generator, snippet_factory_queue)?;
        }

        // insert category into self
        self.categories.insert(category.get_uuid(), category); 

        return Ok(());
    } */

    /*
    /// Create snippets based on their respective paths, running their initialization methods 
    fn create_snippets(&mut self, snippet_factory_queue: &mut Vec<PathBuf>) -> Result<(), String> {
        // Read python code for each snippet path
        // containing the snippet path and python code content
        let mut snippet_python_factory_queue = Vec::<(PathBuf, String)>::new();

        // Only supports one snippet python file at this moment
        for file_path in snippet_factory_queue {
            // Read file
            let file_contents = match fs::read_to_string(file_path) {
                Ok(some) => some,
                Err(e) => {
                    return Err(format!("Error in reading file {} contents durring snippet python factory queue filling", file_path.as_os_str().to_string_lossy()));
                }
            };

            snippet_python_factory_queue.push((file_path.to_owned(), file_contents));
        }

        return Ok(());
    }

    pub fn file_structure_to_front_snippet_contents(&self, visual_directory_component_manager: &mut VisualDirectoryComponentManager, sequential_id_generator: &mut SequentialIdGenerator, external_snippet_manager: &mut ExternalSnippetManager) -> Vec<FrontDirectoryContent> {
        let mut front_snippet_contents: Vec<FrontDirectoryContent> = Vec::with_capacity(self.external_snippet_containers.len());

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
        let front_snippet_content = FrontDirectoryContent::new_category(visual_directory_component_manager, sequential_id_generator, external_snippet_category.get_name(), 0);

        //add to front snippet contents
        front_snippet_contents.push(front_snippet_content);

        self.file_structure_to_front_snippet_contents_helper(visual_directory_component_manager, sequential_id_generator, external_snippet_manager, &mut front_snippet_contents, external_snippet_category, 1);
        //SnippetStructure::file_structure_to_front_snippet_contents_helper(sequential_id_generator, &mut front_snippet_contents, cat, 0);

        return front_snippet_contents;
    }

    /// helper function to snippet_structure_to_front_snippet_contents
    /// recursivly goes though snippet structure
    fn file_structure_to_front_snippet_contents_helper(&self, visual_directory_component_manager: &mut VisualDirectoryComponentManager, sequential_id_generator: &mut SequentialIdGenerator, external_snippet_manager: &mut ExternalSnippetManager, front_snippet_contents: &mut Vec<FrontDirectoryContent>, external_snippet_category: &ExternalSnippetCategory, level: u32) {
        //add external snippets
        for ext_snip_uuid in external_snippet_category.child_snippet_uuids.iter() {
            //find external snippet file container
            let external_snippet_container = self.find_external_snippet_container(ext_snip_uuid).unwrap(); 

            //create front snippet content
            //can safely unwrap since we created the external snippet before this method call
            //nothing else could change the existance or properties of it before
            let front_snippet_content = FrontDirectoryContent::new_snippet(directory_manager, external_snippet_manager, &self, sequential_id_generator, &external_snippet_container, level).unwrap();

            //add to front snippet contents
            front_snippet_contents.push(front_snippet_content)
        }

        //go into external snippet categories
        for cat_uuid in external_snippet_category.get_child_category_uuids().iter() {
            //get external snippet subcategory
            let external_snippet_category = self.find_category(&cat_uuid).unwrap();

            //create front snippet content
            let front_snippet_content = FrontDirectoryContent::new_category(visual_directory_component_manager, sequential_id_generator, external_snippet_category.get_name(), 0);
            //add to front snippet contents
            front_snippet_contents.push(front_snippet_content);

            //call helper to go into category recurrsivly
            self.file_structure_to_front_snippet_contents_helper(visual_directory_component_manager, sequential_id_generator, external_snippet_manager, front_snippet_contents, external_snippet_category, level + 1);
        }
    }*/
    
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

impl ExternalSnippetFileContainer {
    //TODO remove new
    pub fn new(sequential_id_generator: &mut SequentialIdGenerator, parent_category_uuid: Uuid) -> Self {
        return ExternalSnippetFileContainer {
            uuid: sequential_id_generator.get_id(),
            parent_category_uuid: parent_category_uuid,
            python_files: Vec::new()
        };
    }

    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }

    pub fn get_as_front_content(&self, external_snippet_manager: &ExternalSnippetManager, sequential_id_generator: &mut SequentialIdGenerator, level: u32) -> Result<FrontDirectoryContent, &str>{
        //get external snippet 
        let external_snippet = match external_snippet_manager.find_external_snippet(self.get_external_snippet_uuid()) {
            Ok(result) => result,
            Err(e) => {
                return Err("could not find external snippet for uuid in external snippet file container");
            }
        };

        let content = FrontDirectoryContent::new(
            sequential_id_generator.get_id(),
            external_snippet.get_name(),
            self.get_uuid(),
            FrontDirectoryContentType::Snippet,
            false,
            level,
            false,
        );
        
        return Ok(content);
    }
}
*/