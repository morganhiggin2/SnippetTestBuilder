use crate::utils::sequential_id_generator::SequentialIdGenerator;
use crate::core_components::snippet::SnippetManager;

pub struct WindowManager {
    window_sessions: Vec<WindowSession> 
}

struct WindowSession {
    uuid: u32,
    snippet_manager: SnippetManager
}

impl WindowManager {
    /// create a new window session for the window sessions manager
    /// returns uuid of session
    pub fn new_window_session(&mut self, seq_id_gen: &mut SequentialIdGenerator) -> u32 {
        //create new window session
        let window_session = WindowSession::new(seq_id_gen);
        
        //copy window session uuid
        let window_session_uuid = window_session.uuid;

        //add window session to window sessions list
        self.window_sessions.push(window_session);

        //return uuid of window session
        return window_session_uuid;
    }

    /// find a window session in the window manager
    fn find_window_session(&mut self, uuid: u32) -> Option<&mut WindowSession> {
        let window_index_result: Option<usize> = self.window_sessions.iter().position(|w| w.uuid == uuid);

        // handle result cases
        // if found, get index
        // else, return with none
        let window_index: usize = match window_index_result {
            Some(i) => i,
            None => return None 
        };

        //get mutable reference to window session
        let window: &mut WindowSession = &mut self.window_sessions[window_index]; //self.window_sessions.iter().find(|&w| w.uuid == uuid).as_mut();

        //return result
        return Some(window);
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        return WindowManager {
            window_sessions: Vec::new()
        }
    }
}

impl WindowSession {
    /// create a new window session
    pub fn new(seq_id_gen: &mut SequentialIdGenerator) -> Self {
        return WindowSession {
            uuid: seq_id_gen.get_id(),
            snippet_manager: SnippetManager::default()
        }
    }
}

impl Default for WindowSession {
    fn default() -> Self {
        return WindowSession {
            uuid: 0,
            snippet_manager: SnippetManager::default()
        }
    }
}