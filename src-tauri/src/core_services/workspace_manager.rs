use std::path::PathBuf;

use pyo3::ffi::Py_NE;

use super::{
    concurrent_processes::get_projects_directory,
    visual_workspace_component_manager::{FrontWorkspaceContent, VisualWorkspaceComponentManager},
};

pub struct WorkspaceManager {
    root_entry: WorkspaceEntry,
    visual_workspace_manager: VisualWorkspaceComponentManager,
}

pub struct WorkspaceEntry {
    path_id: String,
    entry_type: WorkspaceEntryType,
}

pub enum WorkspaceEntryType {
    WorkspaceParentEntry(WorkspaceParentEntry),
    WorkspaceProjectEntry(WorkspaceProjectEntry),
}

pub struct WorkspaceParentEntry {
    name: String,
    child_entries: Vec<WorkspaceEntry>,
}

pub struct WorkspaceProjectEntry {
    name: String,
}

impl Default for WorkspaceManager {
    fn default() -> Self {
        // return blank workspace with dummy workspace entry
        return Self {
            root_entry: WorkspaceEntry {
                path_id: "".to_string(),
                entry_type: WorkspaceEntryType::WorkspaceParentEntry(WorkspaceParentEntry {
                    name: "".to_string(),
                    child_entries: Vec::new(),
                }),
            },
            visual_workspace_manager: VisualWorkspaceComponentManager::default(),
        };
    }
}

impl WorkspaceManager {
    // Initialized the workspace manager
    pub fn initialize() -> Result<Self, String> {
        let base_path = get_projects_directory();

        // create the necessary directories to ensure this path exists
        if !base_path.exists() {
            match std::fs::create_dir_all(&base_path) {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!(
                        "Failed to create directories {}: {}",
                        base_path.to_string_lossy().to_string(),
                        e
                    ));
                }
            };
        }

        let root_entry =
            WorkspaceManager::recursive_create_workspace_entries(base_path, "".to_string())?;

        return Ok(Self {
            root_entry,
            visual_workspace_manager: VisualWorkspaceComponentManager::default(),
        });
    }

    fn recursive_create_workspace_entries(
        path: PathBuf,
        path_id: String,
    ) -> Result<WorkspaceEntry, String> {
        // get path file name
        let file_name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => {
                return Err(format!(
                    "Unknown error in getting file name from path {}",
                    path.to_string_lossy()
                ))
            }
        };

        // extract file name without extension
        let file_name_without_ext = {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let dot_idx = file_name.rfind('.');
            match dot_idx {
                Some(idx) => file_name[..idx].to_string(),
                None => file_name,
            }
        };

        // create path id
        let mut child_path_id = path_id.to_owned();

        if child_path_id.len() == 0 {
            child_path_id = file_name_without_ext.to_owned();
        } else {
            child_path_id.push('.');
            child_path_id.push_str(&file_name_without_ext);
        }

        // if a directory
        if path.is_dir() {
            // create directory entry
            let mut directory_entry = WorkspaceParentEntry {
                name: file_name,
                child_entries: Vec::new(),
            };

            // for entry in path
            for entry in path
                .read_dir()
                .map_err(|e| format!("Error reading directory: {}", e))?
            {
                // get entry
                let entry = match entry {
                    Ok(some) => some,
                    Err(e) => return Err(format!("Error reading directory entry: {}", e)),
                };

                // get child entry
                let child_entry = WorkspaceManager::recursive_create_workspace_entries(
                    entry.path(),
                    child_path_id.to_owned(),
                )?;

                // push child entries to parent
                directory_entry.child_entries.push(child_entry);
            }

            // return the workspace entry
            return Ok(WorkspaceEntry {
                path_id: child_path_id,
                entry_type: WorkspaceEntryType::WorkspaceParentEntry(directory_entry),
            });
        } else if path.is_file() {
            // create file entry
            let file_entry = WorkspaceProjectEntry {
                name: file_name_without_ext.to_owned(),
            };

            // create path id
            let mut parent_path_id = path_id.to_owned();
            parent_path_id.push_str(&file_name_without_ext);

            // return the workspace entry
            return Ok(WorkspaceEntry {
                path_id: child_path_id,
                entry_type: WorkspaceEntryType::WorkspaceProjectEntry(file_entry),
            });
        } else {
            Err("Path is neither a file nor a directory".to_string())
        }
    }

    pub fn get_root_workspace_entry(&self) -> &WorkspaceEntry {
        return &self.root_entry;
    }

    pub fn get_as_front(&mut self) -> Vec<FrontWorkspaceContent> {
        // borrow split
        let visual_workspace_manager = &self.visual_workspace_manager;
        let root_workspace_entry = self.get_root_workspace_entry();

        return visual_workspace_manager.get_workspace_as_front(root_workspace_entry);
    }
}
impl WorkspaceParentEntry {
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn get_child_entries(&self) -> &Vec<WorkspaceEntry> {
        &self.child_entries
    }
}

impl WorkspaceProjectEntry {
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
}

impl WorkspaceEntry {
    pub fn get_path_id(&self) -> String {
        self.path_id.to_owned()
    }

    pub fn get_entry_type(&self) -> &WorkspaceEntryType {
        &self.entry_type
    }
}
