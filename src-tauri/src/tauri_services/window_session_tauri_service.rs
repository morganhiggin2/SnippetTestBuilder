use crate::{state_management::SharedApplicationState, utils::sequential_id_generator::Uuid};
use std::ops::DerefMut;

///the service for commands between tauri and the front end
#[tauri::command]
pub fn new_window_session(application_state: tauri::State<SharedApplicationState>) -> Uuid {
    // get the state
    let state_guard = &mut application_state.0.lock().unwrap();
    let state = &mut state_guard.deref_mut();

    //borrow split
    let sequential_id_generator = &mut state.sequential_id_generator;
    let window_manager = &mut state.window_manager;

    // create new window session
    let window_id = window_manager.new_window_session(sequential_id_generator);

    return window_id;
}
