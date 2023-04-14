use crate::{state_management::{MutexApplicationState, ApplicationState, window_manager::{WindowManager, WindowSession}, external_snippet_manager::{ExternalSnippetManager, IOContentType}}, core_components::snippet::{SnippetManager, FrontSnippetContent}, utils::sequential_id_generator::{Uuid, self}};
use std::sync::MutexGuard;
use std::ops::DerefMut;

/// create a new snippet
/// get new snippet uuid
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

    //find window session
    let window_session: &mut WindowSession = match window_manager.find_window_session_mut(window_session_uuid) {
        Some(result) => result,
        None => {
            return Err("window session could not be found"); 
        }
    };
    
    //get the external snippet
    let external_snippet = match ext_snippet_manager.find_external_snippet(external_snippet_uuid) {
        Ok(result) => result,
        Err(e) => {
            return Err(e);
        }
    };

    //create snippet
    let snippet_uuid = SnippetManager::new_snippet(seq_id_generator, window_session, external_snippet);

    //return uuid
    return Ok(snippet_uuid.clone());
}

/// get snippet information 
/// 
/// # Arguments
/// * 'window_session_uuid' - uuid of the window session
/// * 'external_snippet_uuid' - uuid of the external snippet it is going to blueprint
#[tauri::command] 
pub fn get_snippet_information(application_state: tauri::State<MutexApplicationState>, window_session_uuid: Uuid, snippet_uuid: Uuid) -> Result<FrontSnippetContent, &str> {
     // get the state
    let mut state_guard: MutexGuard<ApplicationState> = application_state.0.lock().unwrap();
    let state = state_guard.deref_mut();

    //borrow split
    let seq_id_generator = &mut state.seq_id_generator;
    let window_manager = &state.window_manager;

    //find window session
    let window_session: &WindowSession = match window_manager.find_window_session(window_session_uuid) {
        Some(result) => result,
        None => {
            return Err("window session could not be found"); 
        }
    };
    
    let snippet_manager = &window_session.snippet_manager;

    //get the snippet
    let snippet = match snippet_manager.find_snippet(&snippet_uuid) {
        Some(result) => result,
        None => {
            return Err("snippet could not be found from snippet uuid");
        }
    };
    
    //get snippet at front content
    let front_snippet = snippet.get_snippet_to_front_snippet(seq_id_generator, &snippet_manager);
    
    return Ok(front_snippet);

}

/// create new pipeline
/// assumes validate_pipeline has been called, and returned Ok(true)
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
    let window_session: &mut WindowSession = match state.window_manager.find_window_session_mut(window_session_uuid) {
        Some(result) => result,
        None => {
            return Err("window session could not be found"); 
        }
    };

    let mut seq_id_generator = &mut state.seq_id_generator;

    let snippet_manger = &mut window_session.snippet_manager;
    //attempt to create pipeline
    //if could not be created, return none
    let pipeline_uuid: Uuid = match snippet_manger.create_pipeline(&mut seq_id_generator, from_uuid, to_uuid) {
        Ok(result) => result,
        Err(err) => {
            return Err(err);
        }
    };

    return Ok(pipeline_uuid);
}

/// validate a possible pipeline connection
/// from_uuid and to_uuid order/direction not considered
///
/// # Arguments
/// * 'from_uuid' - from pipeline connector uuid
/// * 'to_uuid' - to pipeline connector uuid
#[tauri::command] 
pub fn validate_pipeline_connection(application_state: tauri::State<MutexApplicationState>, window_session_uuid: Uuid, from_uuid: Uuid, to_uuid: Uuid) -> Result<bool, &str> {
    // get the state
    let state_guard = &mut application_state.0.lock().unwrap();
    let state = state_guard.deref_mut();
    
    //find window session
    let window_session: &mut WindowSession = match state.window_manager.find_window_session_mut(window_session_uuid) {
        Some(result) => result,
        None => {
            return Err("window session could not be found"); 
        }
    };

    let snippet_manager = &mut window_session.snippet_manager;

    //validate pipeline, get result
    let result = snippet_manager.validate_pipeline(from_uuid, to_uuid);

    return result;
}

/// check if pipeline connector is already involved in pipeline
/// TODO when connectors can take more than one, this will evolve to handle that
#[tauri::command] 
pub fn check_pipeline_connector_capacity_full(application_state: tauri::State<MutexApplicationState>, window_session_uuid: Uuid, pipeline_connector_uuid: Uuid) -> Result<bool, &str> {
    // get the state
    let state_guard = &mut application_state.0.lock().unwrap();
    let state = state_guard.deref_mut();
    
    //find window session
    let window_session: &mut WindowSession = match state.window_manager.find_window_session_mut(window_session_uuid) {
        Some(result) => result,
        None => {
            return Err("window session could not be found"); 
        }
    };

    let snippet_manager = &mut window_session.snippet_manager;

    let result = snippet_manager.check_pipeline_connector_capacity_full(&pipeline_connector_uuid); 
    
    return Ok(result);
}
/// to get a new unique id
#[tauri::command]
pub fn get_id(application_state: tauri::State<MutexApplicationState>) -> Uuid {
    // get the state
    let state_guard = &mut application_state.0.lock().unwrap();
    let state = state_guard.deref_mut();

    //borrow split
    let seq_id_generator = &mut state.seq_id_generator;

    //get new id
    let id = seq_id_generator.get_id();
    
    return id;
}