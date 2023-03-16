use crate::state_management::MutexApplicationState;
///the service for commands between tauri and the front end
#[tauri::command] 
pub fn new_window_session(application_state: tauri::State<MutexApplicationState>) -> u32 {
    // get the state
    let mut state_guard = application_state.0.lock().unwrap();

    //get mut refs
    let seq_id_generator = state_guard.get_sequence_id_generator();     

    let window_id = state_guard.get_window_manager().new_window_session(seq_id_generator);
    // create new window session

    return window_id;
}