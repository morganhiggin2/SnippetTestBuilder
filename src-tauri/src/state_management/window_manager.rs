use std::sync::MutexGuard;

use crate::utils::sequential_id_generator::{SequentialIdGenerator, self};
use crate::core_components::snippet::SnippetManager;
use crate::state_management::window_manager::visual_component_manager::VisualComponentManager;
use crate::utils::sequential_id_generator::Uuid;

use super::ApplicationState;

pub mod visual_component_manager;

pub struct WindowManager {
    window_sessions: Vec<WindowSession> 
}

pub struct WindowSession {
    pub uuid: Uuid,
    pub snippet_manager: SnippetManager,
    pub visual_component_manager: VisualComponentManager
}

impl WindowManager {
    /// create a new window session for the window sessions manager
    /// returns uuid of session
    pub fn new_window_session(&mut self, seq_id_generator: &mut SequentialIdGenerator) -> Uuid {
        //create new window session
        let window_session = WindowSession::new(seq_id_generator);

        //copy window session uuid
        let window_session_uuid = window_session.uuid;

        //add window session to window sessions list
        self.window_sessions.push(window_session);

        //return uuid of window session
        return window_session_uuid;
    }

    /// find a window session in the window manager
    pub fn find_window_session(&mut self, uuid: u32) -> Option<&mut WindowSession> {
        let window_index_result: Option<usize> = self.window_sessions.iter().position(|w| w.uuid == uuid);

        // handle result cases
        // if found, get index
        // else, return with none
        let window_index: usize = match window_index_result {
            Some(i) => i,
            None => return None 
        };

        //get mutable reference to window session
        let window: &mut WindowSession = self.window_sessions.get_mut(window_index).unwrap(); //self.window_sessions.iter().find(|&w| w.uuid == uuid).as_mut();

        //return result
        return Some(window);
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        return WindowManager {
            window_sessions: Vec::with_capacity(1)
        }
    }
}

impl WindowSession {
    /// create a new window session
    pub fn new(seq_id_generator: &mut SequentialIdGenerator) -> Self {
        return WindowSession {
            uuid: seq_id_generator.get_id(),
            snippet_manager: SnippetManager::default(),
            visual_component_manager: VisualComponentManager::default()
        }
    }
}

impl Default for WindowSession {
    fn default() -> Self {
        return WindowSession {
            uuid: 0,
            snippet_manager: SnippetManager::default(),
            visual_component_manager: VisualComponentManager::default()
        }
    }
}