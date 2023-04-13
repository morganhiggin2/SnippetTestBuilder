use crate::{state_management::{MutexApplicationState}, utils::sequential_id_generator::Uuid};
use std::ops::DerefMut;

///the service for commands between tauri and the front end
#[tauri::command] 
pub fn new_window_session(application_state: tauri::State<MutexApplicationState>) -> Uuid {
    // get the state
    let state_guard = &mut application_state.0.lock().unwrap();
    let state = &mut state_guard.deref_mut();

    //borrow split
    let seq_id_generator = &mut state.seq_id_generator;
    let window_manager = &mut state.window_manager;

    // create new window session
    let window_id = window_manager.new_window_session(seq_id_generator);

    return window_id;
}