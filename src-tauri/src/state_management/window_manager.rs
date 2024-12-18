use serde::Serialize;

use crate::core_components::snippet_manager::SnippetManager;
use crate::core_services::project_service::ProjectManager;
use crate::utils::sequential_id_generator::SequentialIdGenerator;
use crate::utils::sequential_id_generator::Uuid;

use super::visual_snippet_component_manager::VisualSnippetComponentManager;

pub struct WindowManager {
    window_sessions: Vec<WindowSession>,
}

pub struct WindowSession {
    pub uuid: Uuid,
    pub project_manager: ProjectManager,
}

impl WindowManager {
    /// create a new window session for the window sessions manager
    /// returns uuid of session
    pub fn new_window_session(
        &mut self,
        sequential_id_generator: &mut SequentialIdGenerator,
    ) -> Uuid {
        //create new window session
        let window_session = WindowSession::new(sequential_id_generator);

        //copy window session uuid
        let window_session_uuid = window_session.uuid;

        //add window session to window sessions list
        self.window_sessions.push(window_session);

        //return uuid of window session
        return window_session_uuid;
    }

    /// find a reference to a window session in the window manager
    pub fn find_window_session(&self, uuid: u32) -> Option<&WindowSession> {
        let window_index_result: Option<usize> =
            self.window_sessions.iter().position(|w| w.uuid == uuid);

        // handle result cases
        // if found, get index
        // else, return with none
        let window_index: usize = match window_index_result {
            Some(i) => i,
            None => return None,
        };

        //get mutable reference to window session
        let window: &WindowSession = self.window_sessions.get(window_index).unwrap(); //self.window_sessions.iter().find(|&w| w.uuid == uuid).as_mut();

        //return result
        return Some(window);
    }

    /// find a mutable reference window session in the window manager
    pub fn find_window_session_mut(&mut self, uuid: u32) -> Option<&mut WindowSession> {
        let window_index_result: Option<usize> =
            self.window_sessions.iter().position(|w| w.uuid == uuid);

        // handle result cases
        // if found, get index
        // else, return with none
        let window_index: usize = match window_index_result {
            Some(i) => i,
            None => return None,
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
            window_sessions: Vec::with_capacity(1),
        };
    }
}

impl WindowSession {
    /// create a new window session
    pub fn new(sequential_id_generator: &mut SequentialIdGenerator) -> Self {
        return WindowSession {
            uuid: sequential_id_generator.get_id(),
            project_manager: ProjectManager::new(),
        };
    }
}

impl Default for WindowSession {
    fn default() -> Self {
        return WindowSession {
            uuid: 0,
            project_manager: ProjectManager::default(),
        };
    }
}
