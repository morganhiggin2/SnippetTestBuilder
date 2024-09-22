// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::DerefMut;

//use core_services::

use core_services::installation_manager::install_runables;

//use snippet_python_module::python_module::call_init_2;
use crate::state_management::{SharedApplicationState, ApplicationState};
use crate::tauri_services::directory_tauri_service::{get_snippet_directory_details, spawn_initialize_snippet_directory};
use crate::tauri_services::snippet_tauri_service::{new_snippet, validate_pipeline_connection, new_pipeline, check_pipeline_connector_capacity_full, get_id, get_pipeline_connector_uuids_from_pipeline, delete_pipeline, get_snippet_pipelines, get_pipeline_connector_uuids_from_snippet, delete_snippet, update_snippet_parameter_value, spawn_run_snippets};
use crate::tauri_services::window_session_tauri_service::{new_window_session};

pub mod state_management;
pub mod utils;
pub mod core_components;
pub mod tauri_services;
pub mod core_services;
pub mod python_libraries;

fn main() {
    //create application state
    let application_state_guard = SharedApplicationState::default();

    tauri::Builder::default()
        .setup(|app| {
            // initialize installation files 
            install_runables(app);

            return Ok(());
        })
        .manage(application_state_guard)
        .invoke_handler(tauri::generate_handler![logln, new_window_session, new_snippet, validate_pipeline_connection, new_pipeline, check_pipeline_connector_capacity_full, get_id, get_pipeline_connector_uuids_from_pipeline, delete_pipeline, get_snippet_pipelines, get_pipeline_connector_uuids_from_snippet, delete_snippet, spawn_initialize_snippet_directory, get_snippet_directory_details, update_snippet_parameter_value, spawn_run_snippets])
        .run(tauri::generate_context!())
        .expect("error while starting tauri application");
}

#[tauri::command]
fn logln(text: &str) {
    println!("{text}");
}

//IMPORTAINT 
//installed libpython3.10-dev