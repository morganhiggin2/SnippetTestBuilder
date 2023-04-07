use crate::{state_management::{MutexApplicationState, ApplicationState, window_manager::{WindowManager, WindowSession}, external_snippet_manager::{ExternalSnippetManager, IOContentType}}, core_components::snippet::SnippetManager, utils::sequential_id_generator::{Uuid, self}};
use std::sync::MutexGuard;
use std::ops::DerefMut;

/// create new window session
/// 
/// # Arguments
/// * 'window_session_uuid' - uuid of the window session
/// * 'external_snippet_uuid' - uuid of the external snippet it is going to blueprint
#[tauri::command] 
pub fn new_snippet(application_state: tauri::State<MutexApplicationState>, window_session_uuid: Uuid, external_snippet_uuid: Uuid) -> Result<Uuid, &str> {
    // get the state
    let mut state_guard: MutexGuard<ApplicationState> = application_state.0.lock().unwrap();
    let state = state_guard.deref_mut();

    //borrow split
    let seq_id_generator = &mut state.seq_id_generator;
    let window_manager = &mut state.window_manager;
    let ext_snippet_manager = &mut state.external_snippet_manager;

    //get the external snippet
    let external_snippet = match ext_snippet_manager.find_external_snippet(external_snippet_uuid) {
        Ok(result) => result,
        Err(e) => {
            return Err(e);
        }
    };

    //find window session
    let window_session: &mut WindowSession = match WindowManager::find_window_session(window_manager, window_session_uuid) {
        Some(result) => result,
        None => {
            return Err("window session could not be found"); 
        }
    };

    //create snippet
    let snippet_uuid = SnippetManager::new_snippet(seq_id_generator, window_session, external_snippet);

    //return uuid
    return Ok(snippet_uuid.clone());
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
    let state = state_guard.deref_mut();
    
    //find window session
    let window_session: &mut WindowSession = match state.window_manager.find_window_session(window_session_uuid) {
        Some(result) => result,
        None => {
            return Err("window session could not be found"); 
        }
    };

    let mut seq_id_generator = &mut state.seq_id_generator;

    //attempt to create pipeline
    //if could not be created, return none
    let pipeline_uuid: Uuid = match SnippetManager::create_pipeline(&mut seq_id_generator, window_session, from_uuid, to_uuid) {
        Ok(result) => result,
        Err(err) => {
            return Err(err);
        }
    };

    return Ok(pipeline_uuid);
}