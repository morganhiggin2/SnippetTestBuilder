use std::sync::Mutex;
use crate::state_management::window_manager::WindowManager;
use crate::utils::sequential_id_generator::{SequentialIdGenerator};
use crate::state_management::external_snippet_manager::{ExternalSnippetManager};

pub mod window_manager;
pub mod external_snippet_manager;

pub struct MutexApplicationState(pub Mutex<ApplicationState>);
pub struct ApplicationState {
    //sequential id generator
    pub seq_id_generator: SequentialIdGenerator,
    pub window_manager: WindowManager,
    pub external_snippet_manager: ExternalSnippetManager
}

impl Default for ApplicationState {
    /// implement default application state
    fn default() -> ApplicationState {
        return ApplicationState {
            seq_id_generator: SequentialIdGenerator::default(),
            window_manager: WindowManager::default(),
            external_snippet_manager: ExternalSnippetManager::default()
        };

        //TODO delete
         
    }    
}


impl ApplicationState {
    pub fn get_window_manager(&mut self) -> &mut WindowManager {
        return &mut self.window_manager; 
    }

    pub fn get_sequence_id_generator(&mut self) ->  &mut SequentialIdGenerator {
        return  &mut self.seq_id_generator;
    }
}














