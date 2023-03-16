use std::sync::Mutex;
use crate::state_management::window_manager::WindowManager;
use crate::utils::sequential_id_generator::{SequentialIdGenerator, self};

pub mod window_manager;

pub struct MutexApplicationState(pub Mutex<ApplicationState>);
pub struct ApplicationState {
    //sequential id generator
    seq_id_generaor: SequentialIdGenerator,
    window_manager: WindowManager
}

impl Default for ApplicationState {
    /// implement default application state
    fn default() -> ApplicationState {
        return ApplicationState {
            seq_id_generaor: SequentialIdGenerator::default(),
            window_manager: WindowManager::default()
        };
    }    
}


impl ApplicationState {
    pub fn get_window_manager(&mut self) -> &mut WindowManager {
        return &mut self.window_manager; 
    }

    pub fn get_sequence_id_generator(&mut self) ->  &mut SequentialIdGenerator {
        return  &mut self.seq_id_generaor;
    }
}














