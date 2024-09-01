use std::{ffi::OsStr, fs::{self, File}, path::PathBuf};
use enum_as_inner::EnumAsInner;
use std::path::Path;

use crate::{state_management::external_snippet_manager::PackagePath, utils::sequential_id_generator::{SequentialIdGenerator, Uuid}};

use super::{concurrent_processes::get_working_directory, visual_directory_component_manager::{FrontDirectoryContent, VisualDirectoryComponentManager}};

// This here is not ui related
pub struct DirectoryManager {
    //snippet directory
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
    pub fn initialize(&mut self, relative_snippet_directory: &String, sequential_id_generator: &mut SequentialIdGenerator) -> Result<(), String> {
        // if we are already initialized
        if let Some(_) = self.snippet_directory.root {
            // unitialize snippet directory 
            todo!();


            // clear visual component manager
        }

        // First create the snippet directory
        self.snippet_directory.initialize(relative_snippet_directory, sequential_id_generator)?;

        return Ok(());
    }

    /// Get directory manager as front
    pub fn get_as_front(&mut self, sequential_id_generator: &mut SequentialIdGenerator) -> Vec<FrontDirectoryContent> {
        self.visual_component_manager.get_directory_as_front(&self.snippet_directory, sequential_id_generator)
    }

    /// find directory entry from package path
    pub fn find_directory_entry(&self, package_path: PackagePath) -> Option<&SnippetDirectoryEntry> {
        let mut directory_entry = self.snippet_directory.get_root_directory_entry()?;

        for package in package_path.into_iter() {
            // get directory entry type
            match directory_entry.get_inner_as_ref() {
                SnippetDirectoryType::Category(category) => {
                    let mut found_package = false;

                    // search for matching child
                    for child in &category.children {
                        if child.get_name().eq(&package) {
                            // if found, exit inner loop and continue with next package
                            directory_entry = child;
                            found_package = true;
                            break;
                        }
                    }

                    // else, we did not find, exit
                    if !found_package {
                        return None;
                    }
                },
                // nothing else to search
                SnippetDirectoryType::Snippet(_snippet) => (),
            }

        }

        return Some(directory_entry);
    }

    /// return true if the directory manager is initialized, false otherwise
    pub fn is_initialized(&self) -> bool {
        return self.snippet_directory.is_initialized();
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
    pub fn initialize(&mut self, relative_snippet_directory: &String, sequential_id_generator: &mut SequentialIdGenerator) -> Result<(), String> {

        self.scan_and_map_directory(relative_snippet_directory, sequential_id_generator)?;
        
        return Ok(());
    }

    /// reads snippet directory, reads all snippet files,
    /// and compiles all snippet category, file inforation,
    /// as well as assembles external snippets, existing snippets will not be overriden
    /// and new snippets will be inserted
    fn scan_and_map_directory(&mut self, relative_snippet_directory: &String, sequential_id_generator: &mut SequentialIdGenerator) -> Result<(), String> {
        //get current working directory
        let current_working_directory = get_working_directory(); 
        //get snippet directory
        let snippets_directory = current_working_directory.join(relative_snippet_directory);

        // if root category does not exist, override it 
        /*match self.root {
            Some(_) => (), 
            None => {
                // create root category
                let root_category = SnippetDirectoryEntry::new_category("root".to_owned(), snippets_directory.to_owned(), sequential_id_generator); 

                self.root = Some(root_category);
            }
        };*/

        let placeholder_parent_entry = SnippetDirectoryEntry::new_category("placeholder".to_owned(), snippets_directory.to_owned(), sequential_id_generator); 
        let mut placeholder_parent_category = match placeholder_parent_entry.content {
            SnippetDirectoryType::Category(category) => category,
            // Should not be possible, hard coded logic error if so
            SnippetDirectoryType::Snippet(_) => panic!(),
        };

        // walk directory recurrsivly
        SnippetDirectory::directory_walker(&mut placeholder_parent_category, &snippets_directory, sequential_id_generator)?;

        let mut roots: Vec<SnippetDirectoryEntry> = placeholder_parent_category.children.into_iter().filter(|child| -> bool {
            if child.get_name().eq(&"root".to_string()) {
                return match &child.content {
                    SnippetDirectoryType::Category(_) => true,
                    SnippetDirectoryType::Snippet(_) => false,
                }
            }

            false
        })
        .collect();

        // we don't have a root folder
        if roots.len() == 0 {
            return Err("Root root directory missing".to_string());
        }

        // get one that is named root
        self.root = Some(roots.remove(0));
      
        return Ok(());
    }
    // the next step would to call initialize snippet to create snippets on every snippet file container
    // this will create the snippets, and populate the mapping from external snippet front container to external snippets

    /// walk directory method, that looks for snippets, when it reaches a <snippet_name>.py file in the current directory,
    ///   it calls another helper to walk that dirctory for all .py files, list of .py file pathbufs, and 
    ///   by the end of this we have a full directory structure. 
    /// Returns whether or not the entry is or has a child snippet
    fn directory_walker(parent_directory_category: &mut SnippetDirectoryCategory, current_path: &Path, sequential_id_generator: &mut SequentialIdGenerator) -> Result<bool, String> {
        // Get if snippet directory
        let is_snippet_directory = SnippetDirectory::is_directory_snippet(current_path)?;
        // flag for whether or not the the entry is or has a child snippet
        let mut is_parent_category_or_snippet = false;

        let dir_name = match current_path.file_stem() {
            Some(some) => some,
            None => &OsStr::new("")
        };

        let dir_name = match dir_name.to_str() {
            Some(some) => some,
            None => ""
        };

        if is_snippet_directory {
            // if the path does not contain an __init__.py file, add it in the current path
            let mut init_file_path = current_path.to_owned();
            init_file_path.push("__init__.py");
            let _init_file = File::create(init_file_path);

            // create snippet type, add as child 
            let snippet_entry = SnippetDirectoryEntry::new_snippet(dir_name.to_owned(), current_path.to_owned(), sequential_id_generator);

            // add as child
            parent_directory_category.add_child(snippet_entry);

            is_parent_category_or_snippet = true;

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

            // by definition of a directory, we cannot have duplicate entries
        
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
                    is_parent_category_or_snippet = SnippetDirectory::directory_walker(snippet_category, &entry.path(), sequential_id_generator)? || is_parent_category_or_snippet;
                }
            }

            // only if any of the children are snippets, does this qualify as a category
            if is_parent_category_or_snippet {
                // if the path does not contain an __init__.py file, add it in the current path
                let mut init_file_path = current_path.to_owned();
                init_file_path.push("__init__.py");
                let _init_file = File::create(init_file_path);

                // add as child to parent category
                parent_directory_category.add_child(snippet_entry);
            }

        }

        //else this is a random file or something, ignore

        return Ok(is_parent_category_or_snippet);
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

    /// return true if the snippet directory is initialized, false otherwise
    pub fn is_initialized(&self) -> bool {
        if let Some(_) = self.root {
            return true;
        }

        return false;
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

    /// get the runnable python file for the directory manager
    pub fn get_python_file(&self) -> Result<PathBuf, String> {
        if self.path.is_dir() {
            let dir_entries = match fs::read_dir(self.path.to_owned()) {
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
                        return Ok(entry.path());
                    }    
                }
            }

            return Err(format!("app.py file not found for snippet {} at {}, must have been deleted or is hidden", self.get_name(), self.get_path().to_string_lossy()));
        }

        return Err(format!("Snippet {} path is a file, not a directory", self.get_name()));
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    
    use crate::core_services::concurrent_processes::get_working_directory;
    use crate::core_services::directory_manager::{SnippetDirectoryEntry, SnippetDirectoryType};
    use crate::utils::sequential_id_generator::{SequentialIdGenerator};

    use super::SnippetDirectory;

    #[test]
    fn test_scan_and_map_directory() {
        let mut sequential_id_generator = SequentialIdGenerator::default();
        let mut snippet_directory = SnippetDirectory::default();
        let working_directory = get_working_directory();
        
        // We are reading from a sample directory
        snippet_directory.scan_and_map_directory(&"tests/testing_files/sample_directory/data/snippets/root".to_string(), &mut sequential_id_generator).unwrap();

        // Test that the snippet directory was set up accordingly
        let root = match snippet_directory.root {
            Some(entry) => entry,
            None => {
                assert!(false);
                return
            }
        };

        assert_eq!(root.name, "root");
        assert_eq!(root.path, working_directory.join("tests/testing_files/sample_directory/data/snippets/root"));
        
        let mut root_content = match root.content {
            SnippetDirectoryType::Category(category) => category,
            SnippetDirectoryType::Snippet(_) => {
                assert!(false); 
                return
            }
        };

        assert_eq!(root_content.children.len(), 1);

        let main_entry = root_content.children.remove(0); 

        assert_eq!(main_entry.name, "main");
        assert_eq!(main_entry.path, working_directory.join("tests/testing_files/sample_directory/data/snippets/root/main"));

        let main_content = match main_entry.content {
            SnippetDirectoryType::Category(category) => category,
            SnippetDirectoryType::Snippet(_) => {
                assert!(false);
                return
            }
        };

        // if a category has no snippet children, is it not a snippet category
        assert_eq!(main_content.children.len(), 4);

        let children_map: HashMap<String, SnippetDirectoryEntry> = main_content.children.into_iter().map(|element| -> (String, SnippetDirectoryEntry) {
            return (element.name.to_owned(), element);
        })
        .collect();

        // get first child
        let child_one = match children_map.get("basic_one_snippet") {
            Some(entity) => entity,
            None => {
                assert!(false);

                return
            },
        };

        // assert first child is a snippet
        let child_one_content = match &child_one.content {
            SnippetDirectoryType::Category(_) => {
                assert!(false);

                return
            },
            SnippetDirectoryType::Snippet(snippet) => snippet,
        };

        // get second child
        let child_two = match children_map.get("string_operations") {
            Some(entity) => entity,
            None => {
                assert!(false);

                return
            },
        };

        // assert first child is a category 
        let child_two_content = match &child_two.content {
            SnippetDirectoryType::Category(category) => category,
            SnippetDirectoryType::Snippet(_) => {
                assert!(false);

                return
            },
        };

        // child two has only one child 
        assert_eq!(child_two_content.children.len(), 1);
        
        // child two's child
        let child_two_child_only = child_two_content.children.get(0).unwrap();

        assert_eq!(child_two_child_only.name, "remove_index_in_str");

        // get inner snippet, check if it is snippet
        let child_two_child_only_content = match child_two_child_only.content {
            SnippetDirectoryType::Category(_) => assert!(false),
            SnippetDirectoryType::Snippet(_) => {

            },
        };

        // get child three
        let child_three = match children_map.get("math") {
            Some(entity) => entity,
            None => {
                assert!(false);

                return
            },
        };

        // assert first child is a category 
        let child_three_content = match &child_three.content {
            SnippetDirectoryType::Category(category) => category,
            SnippetDirectoryType::Snippet(_) => {
                assert!(false);

                return
            },
        };
        
        assert_eq!(child_three_content.children.len(), 3);

        {
            let children_map: HashMap<String, &SnippetDirectoryEntry> = child_three_content.children.iter().map(|element| -> (String, &SnippetDirectoryEntry) {
                return (element.name.to_owned(), element);
            })
            .collect();

            let child_three_child_one  = match children_map.get("add") {
                Some(entity) => entity,
                None => {
                    assert!(false);

                    return
                },
            };

            // assert first child is a category 
            let child_three_child_one_content = match &child_three_child_one.content {
                SnippetDirectoryType::Category(_) => {
                    assert!(false);

                    return
                },
                SnippetDirectoryType::Snippet(snippet) => snippet,
            };
            
            let child_three_child_two  = match children_map.get("subtract") {
                Some(entity) => entity,
                None => {
                    assert!(false);

                    return
                },
            };

            // assert first child is a category 
            let child_three_child_two_content = match &child_three_child_two.content {
                SnippetDirectoryType::Category(_) => {
                    assert!(false);

                    return
                },
                SnippetDirectoryType::Snippet(snippet) => snippet,
            }; 
            
            let child_three_child_three  = match children_map.get("mul") {
                Some(entity) => entity,
                None => {
                    assert!(false);

                    return
                },
            };

            // assert first child is a category 
            let child_three_child_three_content = match &child_three_child_three.content {
                SnippetDirectoryType::Category(_) => {
                    assert!(false);

                    return
                },
                SnippetDirectoryType::Snippet(snippet) => snippet,
            };
        }

        // get fourth child
        let child_four = match children_map.get("params") {
            Some(entity) => entity,
            None => {
                assert!(false);

                return
            },
        };

        // assert first child is a category 
        let child_four_content = match &child_four.content {
            SnippetDirectoryType::Category(category) => category,
            SnippetDirectoryType::Snippet(_) => {
                assert!(false);

                return
            },
        };

        // child two has only one child 
        assert_eq!(child_four_content.children.len(), 1);
        
        // child two's child
        let child_four_child_only = child_four_content.children.get(0).unwrap();

        assert_eq!(child_four_child_only.name, "str_param");

        // get inner snippet, check if it is snippet
        let child_four_child_only_content = match child_two_child_only.content {
            SnippetDirectoryType::Category(_) => assert!(false),
            SnippetDirectoryType::Snippet(_) => {

            },
        };
    }
}