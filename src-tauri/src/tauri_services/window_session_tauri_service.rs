use crate::state_management::{MutexApplicationState, window_manager::WindowManager};

///the service for commands between tauri and the front end
#[tauri::command] 
pub fn new_window_session(application_state: tauri::State<MutexApplicationState>) -> u32 {
    // get the state
    let state_guard = &mut application_state.0.lock().unwrap();

    // create new window session
    let window_id = WindowManager::new_window_session(state_guard);

    return window_id;
}