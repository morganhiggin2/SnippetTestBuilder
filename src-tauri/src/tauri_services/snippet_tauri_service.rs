use tauri::window;

use crate::{state_management::{MutexApplicationState, ApplicationState, window_manager::{WindowManager, WindowSession}, external_snippet_manager::{ExternalSnippetManager, IOContentType, self}, visual_snippet_component_manager}, core_components::snippet::{SnippetManager, FrontSnippetContent, FrontPipelineContent}, utils::sequential_id_generator::{Uuid, self}, core_services::visual_directory_component_manager};
use std::sync::MutexGuard;
use std::ops::DerefMut;

/// create a new snippet
/// get new snippet uuid
/// 
/// # Arguments
/// * 'window_session_uuid' - uuid of the window session
/// * 'external_snippet_uuid' - uuid of the external snippet it is going to blueprint
#[tauri::command] 
pub fn new_snippet(application_state: tauri::State<MutexApplicationState>, window_session_uuid: Uuid, directory_front_uuid: Uuid) -> Result<FrontSnippetContent, &str> {
    // get the state
    let mut state_guard: MutexGuard<ApplicationState> = application_state.0.lock().unwrap();
    let state = state_guard.deref_mut();

    //borrow split
    let seq_id_generator = &mut state.seq_id_generator;
    let window_manager = &mut state.window_manager;
    let ext_snippet_manager = &mut state.external_snippet_manager;
    let directory_manager = &mut state.directory_manager;

    //find window session
    let window_session: &mut WindowSession = match window_manager.find_window_session_mut(window_session_uuid) {
        Some(result) => result,
        None => {
            return Err("window session could not be found"); 
        }
    };

    //borrow split
    let snippet_manager = &mut window_session.snippet_manager;
    let visual_snippet_component_manager = &mut window_session.visual_component_manager;
    let visual_directory_component_manager = &mut directory_manager.visual_component_manager;
    
    //get file container external snippet uuid from directory front uuid
    let directory_uuid = match visual_directory_component_manager.find_directory_front_uuid(&directory_front_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find directory content uuid with directory front uuid");
        }
    };

    println!("{}", directory_uuid);

    //currently: in creating front extern snippet containers, it is using extern snippet manager, but instead should be using directory uuid

    //get external snippet uuid from directory manager
    let external_snippet_uuid = match directory_manager.snippet_structure.find_external_snippet_container(&directory_uuid) {
        Some(result) => result.get_external_snippet_uuid(),
        None => {
            return Err("external snippet container does not exist for found directory uuid");
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
    let snippet_uuid = snippet_manager.new_snippet(seq_id_generator, external_snippet);

    //get the snippet
    let snippet = match snippet_manager.find_snippet(&snippet_uuid) {
        Some(result) => result,
        None => {
            return Err("snippet could not be found from snippet uuid");
        }
    };
        //get snippet at front content
    let front_snippet = snippet.get_snippet_to_front_snippet(visual_snippet_component_manager, seq_id_generator, &snippet_manager);
    
    //return uuid
    return Ok(front_snippet);
}

/// create new pipeline
/// assumes validate_pipeline has been called, and returned Ok(true)
/// 
/// # Arguments
/// * 'from_front_uuid' - from pipeline connector front uuid
/// * 'to_front_uuid' - to pipeline connector front uuid
#[tauri::command] 
pub fn new_pipeline(application_state: tauri::State<MutexApplicationState>, window_session_uuid: Uuid, from_front_uuid: Uuid, to_front_uuid: Uuid) -> Result<FrontPipelineContent, &str> {
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

    //borrow split
    let mut seq_id_generator = &mut state.seq_id_generator;

    let snippet_manger = &mut window_session.snippet_manager;
    let visual_snippet_component_manager = &mut window_session.visual_component_manager;

    //get from and to component uuids from front uuids
    let from_uuid = match visual_snippet_component_manager.find_pipeline_connector_front_uuid(&from_front_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find from pipeline connector uuids from front pipeline connector uuid");
        }
    };

    let to_uuid = match visual_snippet_component_manager.find_pipeline_connector_front_uuid(&to_front_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find to pipeline connector uuids to front pipeline connector uuid");
        }
    };

    //attempt to create pipeline
    //if could not be created, return none
    let pipeline_uuid: Uuid = match snippet_manger.create_pipeline(&mut seq_id_generator, from_uuid, to_uuid) {
        Ok(result) => result,
        Err(err) => {
            return Err(err);
        }
    };

    //get pipelines, can safely unwrap as we just created the pipeline above
    let pipeline = snippet_manger.find_pipeline(&pipeline_uuid).unwrap();

    //get pipeline front content
    let pipeline_front = pipeline.get_pipeline_as_front_content(seq_id_generator); 

    return Ok(pipeline_front);
}

/// validate a possible pipeline connection
/// from_uuid and to_uuid order/direction not considered
///
/// # Arguments
/// * 'from_fornt_uuid' - from front pipeline connector uuid
/// * 'to_front_uuid' - to front pipeline connector uuid
#[tauri::command] 
pub fn validate_pipeline_connection(application_state: tauri::State<MutexApplicationState>, window_session_uuid: Uuid, from_front_uuid: Uuid, to_front_uuid: Uuid) -> Result<bool, &str> {
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
    let visual_snippet_component_manager = &mut window_session.visual_component_manager;

    //get from and to component uuids from front uuids
    let from_uuid = match visual_snippet_component_manager.find_pipeline_connector_front_uuid(&from_front_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find from pipeline connector uuids from front pipeline connector uuid");
        }
    };

    let to_uuid = match visual_snippet_component_manager.find_pipeline_connector_front_uuid(&to_front_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find to pipeline connector uuids to front pipeline connector uuid");
        }
    };

    //validate pipeline, get result
    let result = snippet_manager.validate_pipeline(from_uuid, to_uuid);

    return result;
}

/// check if pipeline connector is already involved in pipeline
/// TODO when connectors can take more than one, this will evolve to handle that
#[tauri::command] 
pub fn check_pipeline_connector_capacity_full(application_state: tauri::State<MutexApplicationState>, window_session_uuid: Uuid, front_pipeline_connector_uuid: Uuid) -> Result<bool, &str> {
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

    //borrow check
    let snippet_manager = &mut window_session.snippet_manager;
    let visual_snippet_component_manager = &mut window_session.visual_component_manager; 

    //get pipeline connector uuid from front pipeline connector uuid
    let pipeline_connector_uuid = match visual_snippet_component_manager.find_pipeline_connector_uuid(&front_pipeline_connector_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find pipeline connector uuid from front pipeline connector uuid");
        } 
    };

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