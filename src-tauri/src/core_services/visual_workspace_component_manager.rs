use bimap::BiHashMap;
use serde::{Deserialize, Serialize};

use crate::utils::sequential_id_generator::{SequentialIdGenerator, Uuid};

use super::workspace_manager::{WorkspaceEntry, WorkspaceEntryType, WorkspaceManager};

pub struct VisualWorkspaceComponentManager {}

impl Default for VisualWorkspaceComponentManager {
    fn default() -> Self {
        return VisualWorkspaceComponentManager {};
    }
}

//struct for the josn serialization
#[derive(Serialize, Deserialize)]
pub struct FrontWorkspaceContent {
    // the id is going to be the path id
    id: String,
    name: String,
    project_display_id: String,
    file_type: FrontWorkspaceContentType,
    level: u32,
    showing: bool,
}

#[derive(Serialize, Deserialize)]
pub enum FrontWorkspaceContentType {
    Parent,
    Project,
}

impl VisualWorkspaceComponentManager {
    /// Get the workspace as front elements
    /// These are displayed in descencing order, expandable and contractable based on parent
    /// If the workspace were to get reloaded, we would need to reload the frontworkspace
    pub fn get_workspace_as_front(
        &self,
        root_workspace_entry: &WorkspaceEntry,
    ) -> Vec<FrontWorkspaceContent> {
        // Walk workspace recursivly, keeping track of level, calling VisualWorkspaceComponentManager::new_from_workspace_entry
        let mut front_workspace_content = Vec::<FrontWorkspaceContent>::new();

        // starting leve is 1, but we are going to start at 0 as the base level is the root directory
        self.front_workspace_walker(root_workspace_entry, &mut front_workspace_content, 0);

        // remove root directory
        front_workspace_content.remove(0);

        // set any entry with level 1 to showing
        front_workspace_content
            .iter_mut()
            .for_each(|front_workspace_entry| {
                if front_workspace_entry.level == 1 {
                    front_workspace_entry.showing = true;
                }
            });

        // Remove root from workspace fronts, which will be the first element
        return front_workspace_content;
    }

    fn front_workspace_walker(
        &self,
        root_workspace_entry: &WorkspaceEntry,
        front_workspace_content: &mut Vec<FrontWorkspaceContent>,
        level: u32,
    ) {
        let path_id = root_workspace_entry.get_path_id();

        // remove project parent part from name
        let path_display_id = path_id.trim_start_matches("projects.").to_string();

        match root_workspace_entry.get_entry_type() {
            WorkspaceEntryType::WorkspaceParentEntry(parent_workspace_entry) => {
                // create parent front entry
                let front_workspace_entry = FrontWorkspaceContent {
                    id: path_id,
                    name: parent_workspace_entry.get_name(),
                    project_display_id: path_display_id,
                    file_type: FrontWorkspaceContentType::Parent,
                    level: level,
                    showing: false,
                };

                // add to front workspace list
                front_workspace_content.push(front_workspace_entry);

                // Call children of category workspace entry
                for child_workspace_entry in parent_workspace_entry.get_child_entries() {
                    self.front_workspace_walker(
                        child_workspace_entry,
                        front_workspace_content,
                        level + 1,
                    );
                }
            }
            WorkspaceEntryType::WorkspaceProjectEntry(project_workspace_entry) => {
                // create parent front entry
                let front_workspace_entry = FrontWorkspaceContent {
                    id: path_id,
                    name: project_workspace_entry.get_name(),
                    project_display_id: path_display_id,
                    file_type: FrontWorkspaceContentType::Project,
                    level: level,
                    showing: false,
                };

                // add to front workspace list
                front_workspace_content.push(front_workspace_entry);
            }
        };
    }
}
