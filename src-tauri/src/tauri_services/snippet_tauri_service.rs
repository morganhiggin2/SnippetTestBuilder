use serde::Serialize;

use crate::{core_services::{concurrent_processes::spawn_run_snippets_event, directory_manager}, python_libraries::python_run_module::InitializedPythonSnippetRunnerBuilder, state_management::{external_snippet_manager, visual_snippet_component_manager::{FrontPipelineContent, FrontSnippetContent}, window_manager::WindowSession, ApplicationState, SharedApplicationState}, utils::sequential_id_generator::Uuid};
use std::sync::{Arc, MutexGuard};
use std::ops::DerefMut;

/// create a new snippet
/// get new snippet uuid
/// 
/// # Arguments
/// * 'window_session_uuid' - uuid of the window session
/// * 'external_snippet_uuid' - uuid of the external snippet it is going to blueprint
#[tauri::command] 
pub fn new_snippet(application_state: tauri::State<SharedApplicationState>, window_session_uuid: Uuid, directory_front_uuid: Uuid) -> Result<FrontSnippetContent, &str> {
    // get the state
    let mut state_guard: MutexGuard<ApplicationState> = application_state.0.lock().unwrap();
    let state = state_guard.deref_mut();

    //borrow split
    let sequential_id_generator = &mut state.sequential_id_generator;
    let window_manager = &mut state.window_manager;
    let external_snippet_manager = &mut state.external_snippet_manager;
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
    let directory_uuid = match visual_directory_component_manager.find_directory_entry_uuid(&directory_front_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find directory content uuid with directory front uuid");
        }
    };

    //get external snippet from directory uuid
    let external_snippet = match external_snippet_manager.find_external_snippet_from_directory_uuid(directory_uuid) {
        Some(result) => result,
        None => {
            return Err("external snippet container does not exist for found directory uuid");
        }
    };

    //create snippet
    let snippet_uuid = snippet_manager.new_snippet(sequential_id_generator, external_snippet);

    //get the snippet
    let snippet = match snippet_manager.find_snippet(&snippet_uuid) {
        Some(result) => result,
        None => {
            return Err("snippet could not be found from snippet uuid");
        }
    };

    //get snippet at front content and add to virtual manager
    let front_snippet = snippet.get_snippet_to_front_snippet(visual_snippet_component_manager, sequential_id_generator, &snippet_manager);
    
    //return uuid
    return Ok(front_snippet);
}

/// get front pipeline connector uuids for front pipeline uuid
/// 
/// # Arguments 
/// * 'front_uuid' - uuid of snippet 
#[tauri::command]
pub fn get_pipeline_connector_uuids_from_snippet(application_state: tauri::State<SharedApplicationState>, window_session_uuid: Uuid, front_uuid: Uuid) -> Result<Vec<Uuid>, &str> {
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
    let snippet_manager = &mut window_session.snippet_manager;
    let visual_snippet_component_manager = &mut window_session.visual_component_manager;

    //get pipeline uuid from front uuid
    let snippet_uuid = match visual_snippet_component_manager.find_snippet_uuid(&front_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find pipeline uuid from front pipeline uuid");
        }
    };

    //find pipieline
    let snippet_component = match snippet_manager.find_snippet(&snippet_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find pipeline from pipeline uuid");
        }
    };
    
    //get uuids
    let pipeline_connector_uuids = snippet_component.get_pipeline_connector_uuids();

    //get front uuids from pipeline connectors
    let mut front_pipeline_connector_uuids: Vec<Uuid> = Vec::with_capacity(pipeline_connector_uuids.len());

    for pipeline_connector_uuid in pipeline_connector_uuids {
        let front_pipeline_connector_uuid = match visual_snippet_component_manager.find_pipeline_connector_front_uuid(&pipeline_connector_uuid) {
            Some(result) => result,
            None => {
                return Err("pipeline connector uuid does not exist in visual snippet manager");
            }
        };

        front_pipeline_connector_uuids.push(front_pipeline_connector_uuid);
    }

    return Ok(front_pipeline_connector_uuids);
}

/// updates the snippet parameter value
/// including all front and root components 
/// 
/// # Arguments 
/// * 'front_uuid' - uuid of the snippet
/// * 'value' - value of the parameter in string form
#[tauri::command] 
pub fn update_snippet_parameter_value(application_state: tauri::State<SharedApplicationState>, window_session_uuid: Uuid, front_uuid: Uuid, value: String) -> Result<(), &str> {
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
    let snippet_manager = &mut window_session.snippet_manager;
    let visual_snippet_component_manager = &mut window_session.visual_component_manager;

    // get the snippet parameter uuid from the front parameter uuid
    let parameter_uuid = match visual_snippet_component_manager.find_parameter_uuid_from_parameter_front(front_uuid) {
        Some(some) => some,
        None => {
            return Err("could not find parameter front in visual snippet component manager");
        },
    };

    // find parameter from parameter uuid
    let parameter = match snippet_manager.find_parameter(&parameter_uuid) {
        Some(some) => some,
        None => {
            return Err("could not find parameter in snippet manager");
        }
    };

    // update value in parameter
    match parameter.update_value(value) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        },
    };

    return Ok(());
}

/// deletes snippet
/// including all front and root components 
/// 
/// # Arguments 
/// * 'front_uuid' - uuid of the snippet
#[tauri::command] 
pub fn delete_snippet(application_state: tauri::State<SharedApplicationState>, window_session_uuid: Uuid, front_uuid: Uuid) -> Result<(), &str> {
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
    let snippet_manager = &mut window_session.snippet_manager;
    let visual_snippet_component_manager = &mut window_session.visual_component_manager;

    //get snippet uuid from front uuid
    let snippet_uuid = match visual_snippet_component_manager.find_snippet_uuid(&front_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find snippet uuid from front snippet uuid");
        }
    };

    //get snippet component
    let snippet_component = match snippet_manager.find_snippet(&snippet_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find snippet component from snippet uuid");
        }
    };

    //get pipeline connectors in snippet
    let pipeline_connector_uuids = snippet_component.get_pipeline_connector_uuids();

    //get pipelines from pipeline connectors
    for pipeline_connector_uuid in pipeline_connector_uuids.iter() {
        //if any pipelines are left connected, return error, cannot delete snippet if connected
        match snippet_manager.find_pipeline(&pipeline_connector_uuid) {
            Some(_) => {
                return Err("cannot delete snippet becuase pipelines are still connected to this snippet");
            },
            None => {
                continue;
            }
        };
    }

    //delete pipeline connectors from fornt service
    for pipeline_connector_uuid in pipeline_connector_uuids.iter() {
        match visual_snippet_component_manager.delete_pipeline_connector_by_internal(&pipeline_connector_uuid) {
            Ok(_) => (),
            Err(err) => {
                return Err(err);
            }
        }
    } 

    //delete snippet from front service
    match visual_snippet_component_manager.delete_snippet_by_internal(&snippet_uuid) {
        Ok(_) => (),
        Err(err) => {
            return Err(err);
        }
    };

    //delete snippet from snippet manager
    match snippet_manager.delete_snippet(&snippet_uuid) {
        Ok(_) => (),
        Err(err) => {
            return Err(err);
        }
    };

    return Ok(());


}

/// create new pipeline
/// assumes validate_pipeline has been called, and returned Ok(true)
/// 
/// # Arguments
/// * 'from_front_uuid' - from pipeline connector front uuid
/// * 'to_front_uuid' - to pipeline connector front uuid
#[tauri::command] 
pub fn new_pipeline(application_state: tauri::State<SharedApplicationState>, window_session_uuid: Uuid, from_front_uuid: Uuid, to_front_uuid: Uuid) -> Result<FrontPipelineContent, &str> {
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
    let mut sequential_id_generator = &mut state.sequential_id_generator;

    let snippet_manager = &mut window_session.snippet_manager;
    let visual_snippet_component_manager = &mut window_session.visual_component_manager;

    //get from and to component uuids from front uuids
    let from_uuid = match visual_snippet_component_manager.find_pipeline_connector_uuid(&from_front_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find from pipeline connector uuids from front pipeline connector uuid");
        }
    };

    let to_uuid = match visual_snippet_component_manager.find_pipeline_connector_uuid(&to_front_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find to pipeline connector uuids to front pipeline connector uuid");
        }
    };

    //attempt to create pipeline
    //if could not be created, return none
    let pipeline_uuid: Uuid = match snippet_manager.create_pipeline(&mut sequential_id_generator, from_uuid, to_uuid) {
        Ok(result) => result,
        Err(err) => {
            return Err(err);
        }
    };

    // get pipelines, can safely unwrap as we just created the pipeline above
    let pipeline = snippet_manager.find_pipeline(&pipeline_uuid).unwrap();

    // get pipeline front content and add to virtaul manager
    let pipeline_front = pipeline.create_pipeline_as_front_content(visual_snippet_component_manager, sequential_id_generator); 

    return Ok(pipeline_front);
}

#[tauri::command] 
pub fn delete_pipeline(application_state: tauri::State<SharedApplicationState>, window_session_uuid: Uuid, front_uuid: Uuid) -> Result<(), & str> {
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
    let snippet_manager = &mut window_session.snippet_manager;
    let visual_snippet_component_manager = &mut window_session.visual_component_manager;

    //get pipeline uuid from front uuid
    let pipeline_uuid = match visual_snippet_component_manager.find_pipeline_uuid(&front_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find pipeline uuid from front pipeline uuid");
        }
    };

    //delete from visual snippet manager
    match visual_snippet_component_manager.delete_pipeline_by_pipeline_front(&front_uuid) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    };

    //delete from snippet manager and it's internal links
    match snippet_manager.delete_pipeline(&pipeline_uuid) {
        Ok(_) => (),
        Err(err) => {
            return Err(err);
        }
    };

    return Ok(());
}

/// validate a possible pipeline connection
/// from_uuid and to_uuid order/direction not considered
///
/// # Arguments
/// * 'from_fornt_uuid' - from front pipeline connector uuid
/// * 'to_front_uuid' - to front pipeline connector uuid
#[tauri::command] 
pub fn validate_pipeline_connection(application_state: tauri::State<SharedApplicationState>, window_session_uuid: Uuid, from_front_uuid: Uuid, to_front_uuid: Uuid) -> Result<bool, &str> {
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
    let from_uuid = match visual_snippet_component_manager.find_pipeline_connector_uuid(&from_front_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find from pipeline connector uuids from front pipeline connector uuid");
        }
    };

    let to_uuid = match visual_snippet_component_manager.find_pipeline_connector_uuid(&to_front_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find to pipeline connector uuids to front pipeline connector uuid");
        }
    };

    //validate pipeline, get result
    let result = snippet_manager.validate_pipeline(from_uuid, to_uuid);

    return result;
}

/// get pipelines associated with snippet
#[tauri::command] 
pub fn get_snippet_pipelines(application_state: tauri::State<SharedApplicationState>, window_session_uuid: Uuid, snippet_front_uuid: Uuid) -> Result<Vec<Uuid>, &str> {
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

    //get snippet uuid from front uuid
    let snippet_uuid = match visual_snippet_component_manager.find_snippet_uuid(&snippet_front_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find snippet uuid from front snippet uuid");
        }
    };

    //get snippet component
    let snippet_component = match snippet_manager.find_snippet(&snippet_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find snippet component from snippet uuid");
        }
    };

    //get pipeline connectors in snippet
    let pipeline_connector_uuids = snippet_component.get_pipeline_connector_uuids();
 
    //get pipelines associated with pipeline connectors
    //list of front pipeline uuids soon to be filled
    let mut front_pipelines_uuid: Vec<Uuid> = Vec::with_capacity(pipeline_connector_uuids.len());

    // because no one pipeline can occupy more than one connector on a snippet, we are guanteed they are all different pipelines
    for pipeline_connector_uuid in pipeline_connector_uuids {
        //get pipeline component uuid
        let pipeline_component_uuids = snippet_manager.find_pipeline_uuids_from_pipeline_connector(&pipeline_connector_uuid);       

        // for each 
        for pipeline_component_uuid in pipeline_component_uuids.iter() {
            //get pipeline front uuid
            let pipeline_front_uuid = match visual_snippet_component_manager.find_pipeline_front_uuid(&pipeline_component_uuid) {
                Some(result) => result,
                None => {
                    return Err("could not find pipeline connector in visual manager");
                }
            };

            //add to list of front uuids
            front_pipelines_uuid.push(pipeline_front_uuid);
        };
    }

    //return 
    return Ok(front_pipelines_uuid);
}

/// check if pipeline connector is already involved in pipeline
/// TODO when connectors can take more than one, this will evolve to handle that
#[tauri::command] 
pub fn check_pipeline_connector_capacity_full(application_state: tauri::State<SharedApplicationState>, window_session_uuid: Uuid, front_pipeline_connector_uuid: Uuid) -> Result<bool, &str> {
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

#[derive(Serialize)]
pub struct FrontPipelineConnectorResult {
    front_from_pipeline_connector_uuid: Uuid,
    front_to_pipeline_connector_uuid: Uuid
}

/// get front pipeline connector uuids for front pipeline uuid
#[tauri::command]
pub fn get_pipeline_connector_uuids_from_pipeline(application_state: tauri::State<SharedApplicationState>, window_session_uuid: Uuid, front_pipeline_uuid: Uuid) -> Result<FrontPipelineConnectorResult, &str> {
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
    let snippet_manager = &mut window_session.snippet_manager;
    let visual_snippet_component_manager = &mut window_session.visual_component_manager;

    //get pipeline uuid from front uuid
    let pipeline_uuid = match visual_snippet_component_manager.find_pipeline_uuid(&front_pipeline_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find pipeline uuid from front pipeline uuid");
        }
    };


    //find pipieline
    let pipeline_component = match snippet_manager.find_pipeline(&pipeline_uuid) {
        Some(result) => result,
        None => {
            return Err("could not find pipeline from pipeline uuid");
        }
    };
    
    //get uuids

    //get front from pipeline connector uuid
    let front_from_pipeline_connector_uuid = match visual_snippet_component_manager.find_pipeline_connector_front_uuid(&pipeline_component.get_from_pipeline_connector_uuid()) {
        Some(result) => result,
        None => {
            return Err("could not find front from pipeline connector uuid from pipeline component");
        }
    };

    //get front to pipeline connector uuid
    let front_to_pipeline_connector_uuid = match visual_snippet_component_manager.find_pipeline_connector_front_uuid(&pipeline_component.get_to_pipeline_connector_uuid()) {
        Some(result) => result,
        None => {
            return Err("could not find front to pipeline connector uuid from pipeline component");
        }
    };

    //create result
    let result = FrontPipelineConnectorResult {
        front_from_pipeline_connector_uuid: front_from_pipeline_connector_uuid,
        front_to_pipeline_connector_uuid: front_to_pipeline_connector_uuid
    };

    return Ok(result);
}

/// to get a new unique id
#[tauri::command]
pub fn get_id(application_state: tauri::State<SharedApplicationState>) -> Uuid {
    // get the state
    let state_guard = &mut application_state.0.lock().unwrap();
    let state = state_guard.deref_mut();

    //borrow split
    let sequential_id_generator = &mut state.sequential_id_generator;

    //get new id
    let id = sequential_id_generator.get_id();
    
    return id;
}

/// spawn run snippets
#[tauri::command]
pub fn spawn_run_snippets(application_state: tauri::State<SharedApplicationState>, app_handle: tauri::AppHandle, window_session_uuid: Uuid) -> Result<u32, String> {
    // get the state
    let mut state_guard: MutexGuard<ApplicationState> = application_state.0.lock().unwrap();
    let state = state_guard.deref_mut();

    // create log file and stream from window uuid
    // that way the log instance is specific to the window uuid
    let logging_instance = state.logging_manager.create_new_stream(app_handle, window_session_uuid).unwrap();
    let stream_i = logging_instance.get_stream_i();

    // get shared reference to state 
    // note this is a custom clone implementation utilizing on arc::clone
    //let application_state_ref : SharedApplicationState = SharedApplicationState(Arc::clone(&application_state.0));

    // lock the application state

    let external_snippet_manager = &mut state.external_snippet_manager;
    let sequential_id_generator = &mut state.sequential_id_generator;
    let directory_manager = &mut state.directory_manager;
    //find window session
    let window_session = match state.window_manager.find_window_session_mut(window_session_uuid) {
        Some(result) => result,
        None => {
            return Err("window session could not be found".to_string()); 
        }
    };

    let snippet_manager = &mut window_session.snippet_manager;
    let visual_snippet_component_manager = &mut window_session.visual_component_manager;

    // create build initialized state
    let build_state = match InitializedPythonSnippetRunnerBuilder::build(snippet_manager, external_snippet_manager, directory_manager, visual_snippet_component_manager, sequential_id_generator) {
        Ok(some) => some,
        Err(e) => {
            return Err(format!("Failed to initialize run state: {}", e));
        }
    };

    // spawn process, passing ownership of shared application state
    tauri::async_runtime::spawn(async move {
        spawn_run_snippets_event(build_state, logging_instance).await;     
    });

    return Ok(stream_i);
}
