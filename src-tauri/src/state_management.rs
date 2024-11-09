use std::sync::{Arc, Mutex};

use crate::core_services::directory_manager::DirectoryManager;
use crate::core_services::runtime_logging_service::LoggingStreamManager;
use crate::core_services::workspace_manager::WorkspaceManager;
use crate::state_management::external_snippet_manager::ExternalSnippetManager;
use crate::state_management::window_manager::WindowManager;
use crate::utils::sequential_id_generator::SequentialIdGenerator;
//use crate::core_services::python_service::{call_init_todo_delete_this_method};

pub mod external_snippet_manager;
pub mod visual_snippet_component_manager;
pub mod window_manager;

pub struct SharedApplicationState(pub Arc<Mutex<ApplicationState>>);

pub struct ApplicationState {
    //sequential id generator
    pub sequential_id_generator: SequentialIdGenerator,
    pub logging_manager: LoggingStreamManager,
    pub window_manager: WindowManager,
    pub external_snippet_manager: ExternalSnippetManager,
    pub directory_manager: DirectoryManager,
    pub workspace_manager: WorkspaceManager,
}

impl Default for SharedApplicationState {
    fn default() -> Self {
        return SharedApplicationState(Arc::new(Mutex::new(ApplicationState::default())));
    }
}

impl Clone for SharedApplicationState {
    fn clone(&self) -> Self {
        return SharedApplicationState(Arc::clone(&self.0));
    }
}

impl Default for ApplicationState {
    /// implement default application state
    fn default() -> ApplicationState {
        return ApplicationState {
            sequential_id_generator: SequentialIdGenerator::default(),
            logging_manager: LoggingStreamManager::default(),
            window_manager: WindowManager::default(),
            external_snippet_manager: ExternalSnippetManager::default(),
            directory_manager: DirectoryManager::default(),
            workspace_manager: WorkspaceManager::default(),
        };
    }
}

impl ApplicationState {
    pub fn get_window_manager(&mut self) -> &mut WindowManager {
        return &mut self.window_manager;
    }

    pub fn get_sequence_id_generator(&mut self) -> &mut SequentialIdGenerator {
        return &mut self.sequential_id_generator;
    }
}
