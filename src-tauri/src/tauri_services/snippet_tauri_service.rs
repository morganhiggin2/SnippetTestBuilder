use crate::{state_management::{MutexApplicationState, ApplicationState, window_manager::{WindowManager, WindowSession}}, core_components::snippet::SnippetManager, utils::sequential_id_generator::Uuid};
use std::sync::MutexGuard;

/// create new window session
#[tauri::command] 
pub fn new_snippet(application_state: tauri::State<MutexApplicationState>, window_session_uuid: Uuid) -> Result<Uuid, &str> {
    // get the state
    let state_guard = &mut application_state.0.lock().unwrap();

    //find window session
    let window_session: &mut WindowSession = match state_guard.window_manager.find_window_session(window_session_uuid) {
        Some(result) => result,
        None => {
            return Err("window session could not be found"); 
        }
    };

    //create snippet
    let snippet_uuid = SnippetManager::new_snippet(state_guard, window_session);

    //return uuid
    return Ok(snippet_uuid);
}

/// create new pipeline
/// 
/// # Arguments
/// * 'from_uuid' - from pipeline connector uuid
/// * 'to_uuid' - to pipeline connector uuid
#[tauri::command] 
pub fn new_pipeline(application_state: tauri::State<MutexApplicationState>, window_session_uuid: Uuid, from_uuid: Uuid, to_uuid: Uuid) -> Result<Uuid, &str> {
     // get the state
    let state_guard = &mut application_state.0.lock().unwrap();

    //find window session
    let window_session: &mut WindowSession = match state_guard.window_manager.find_window_session(window_session_uuid) {
        Some(result) => result,
        None => {
            return Err("window session could not be found"); 
        }
    };

    //attempt to create pipeline
    //if could not be created, return none
    let pipeline_uuid: Uuid = match SnippetManager::create_pipeline(from_uuid, to_uuid) {
        Ok(result) => result,
        Err(err) => {
            return Err(err);
        }
    };


}