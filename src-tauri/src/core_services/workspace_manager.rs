use std::path::PathBuf;

use super::concurrent_processes::get_projects_directory;

pub struct WorkspaceManager {
    root_entry: WorkspaceEntry,
}

enum WorkspaceEntry {
    WorkspaceParentEntry(WorkspaceParentEntry),
    WorkspaceProjectEntry(WorkspaceProjectEntry),
}
struct WorkspaceParentEntry {
    name: String,
    child_entries: Vec<WorkspaceEntry>,
}

struct WorkspaceProjectEntry {
    name: String,
}

impl Default for WorkspaceManager {
    fn default() -> Self {
        // return blank workspace with dummy workspace entry
        return Self {
            root_entry: WorkspaceEntry::WorkspaceParentEntry(WorkspaceParentEntry {
                name: "".to_string(),
                child_entries: Vec::new(),
            }),
        };
    }
}

impl WorkspaceManager {
    // Initialized the workspace manager
    pub fn initialize() -> Result<Self, String> {
        let base_path = get_projects_directory();

        let root_entry = WorkspaceManager::recursive_create_workspace_entries(base_path)?;

        return Ok(Self { root_entry });
    }

    fn recursive_create_workspace_entries(path: PathBuf) -> Result<WorkspaceEntry, String> {
        // if a directory
        if path.is_dir() {
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
                let child_entry =
                    WorkspaceManager::recursive_create_workspace_entries(entry.path())?;

                // push child entries to parent
                directory_entry.child_entries.push(child_entry);
            }

            // return the workspace entry
            return Ok(WorkspaceEntry::WorkspaceParentEntry(directory_entry));
        } else if path.is_file() {
            // create file entry
            let file_entry = WorkspaceProjectEntry {
                name: path.file_name().unwrap().to_string_lossy().to_string(),
            };

            // return the workspace entry
            return Ok(WorkspaceEntry::WorkspaceProjectEntry(file_entry));
        } else {
            Err("Path is neither a file nor a directory".to_string())
        }
    }
}
