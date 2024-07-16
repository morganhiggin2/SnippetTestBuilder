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

impl Default for DirectoryManager {
    fn default() -> Self {
        DirectoryManager {
            snippet_directory: SnippetDirectory::default(),
            visual_component_manager: VisualDirectoryComponentManager::default()
        }
    }
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
                let root_category = SnippetDirectoryEntry::new_category("root".to_owned(), snippets_directory.to_owned(), sequential_id_generator); 

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
                    SnippetDirectory::directory_walker(snippet_category, &entry.path(), sequential_id_generator)?;
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

                // check if this is a .py file
                if let Some(file_extension) = entry.path().extension(){
                    // get file name
                    let file_name = entry.path().file_stem().unwrap_or_default().to_owned();

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

    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
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