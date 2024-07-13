// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::DerefMut;

//use core_services::

//use snippet_python_module::python_module::call_init_2;
use crate::state_management::{MutexApplicationState, ApplicationState};
use crate::tauri_services::directory_tauri_service::get_snippet_directory;
use crate::tauri_services::snippet_tauri_service::{new_snippet, validate_pipeline_connection, new_pipeline, check_pipeline_connector_capacity_full, get_id, get_pipeline_connector_uuids_from_pipeline, delete_pipeline, get_snippet_pipelines, get_pipeline_connector_uuids_from_snippet, delete_snippet};
use crate::tauri_services::window_session_tauri_service::{new_window_session};

pub mod state_management;
pub mod utils;
pub mod core_components;
pub mod tauri_services;
pub mod core_services;
pub mod python_libraries;

fn main() {
    /*match call_init() {
        Ok(_) => (),
        Err(e) => {
            println!("error {}", e);
        }
    }*/
    //create application state
    let application_state_guard = MutexApplicationState::default();
        
    {
        let mut guard = application_state_guard.0.lock().unwrap();
        let app_ref = guard.deref_mut();

        //call init for utils and services first

        //call init for state management system
        ApplicationState::init(app_ref);

        /*match call_init(&mut app_ref.sequential_id_generator, &mut app_ref.external_snippet_manager) {
            Ok(_) => (),
            Err(e) => {
                println!("error {}", e);
            }
        }*/
    }

    tauri::Builder::default()
        .manage(application_state_guard)
        .invoke_handler(tauri::generate_handler![logln, new_window_session, get_snippet_directory, new_snippet, validate_pipeline_connection, new_pipeline, check_pipeline_connector_capacity_full, get_id, get_pipeline_connector_uuids_from_pipeline, delete_pipeline, get_snippet_pipelines, get_pipeline_connector_uuids_from_snippet, delete_snippet])
        .run(tauri::generate_context!())
        .expect("error while starting tauri application");
}

#[tauri::command]
fn logln(text: &str) {
    println!("{text}");
}

//TODO write test cases

//std::process:Command

//IMPORTAINT 
//installed libpython3.10-dev

//COULD DO
// trait NEW that mandiates the seq id generator with a new() function



/*
1. be able to delete snippets - done
2. be able to delete pipelines / io lines - done
3. prevent cycles (so this must be a dag)
4. unit tests
3. snip on grid
4. visual pipelines are filled in grid spots 
  - but don't show grid lines, so ontop
  - can represent as multiple lines
5. parameter snippets
  - build in
  - add to directory structure
6. ui for parameters
7. python console output to rust output stream
  - custom os stream
  - wrapper for run function that catpures input and errors, and calls rust to output it


*/