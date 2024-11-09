// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::DerefMut;

//use core_services::

use core_services::installation_manager::install_runables;

//use snippet_python_module::python_module::call_init_2;
use crate::state_management::{ApplicationState, SharedApplicationState};
use crate::tauri_services::directory_tauri_service::{
    get_snippet_directory_details, get_workspace_details,
    spawn_initialize_snippet_directory_and_workspace,
};
use crate::tauri_services::project_tauri_service::{
    get_directory_id_from_package_path, get_front_parameter_id_from_snippet_uuid_and_name,
    get_front_snippet_connector_id_from_snippet_uuid_and_name, open_project, save_project,
};
use crate::tauri_services::snippet_tauri_service::{
    check_pipeline_connector_capacity_full, delete_pipeline, delete_snippet, get_id,
    get_pipeline_connector_uuids_from_pipeline, get_pipeline_connector_uuids_from_snippet,
    get_snippet_pipelines, new_pipeline, new_snippet, spawn_run_snippets,
    update_snippet_parameter_value, update_snippet_position, validate_pipeline_connection,
};
use crate::tauri_services::window_session_tauri_service::new_window_session;

pub mod core_components;
pub mod core_services;
pub mod python_libraries;
pub mod state_management;
pub mod tauri_services;
pub mod utils;

fn main() {
    //create application state
    let application_state_guard = SharedApplicationState::default();

    // install necessary python runable files
    install_runables();

    tauri::Builder::default()
        .setup(|_app| return Ok(()))
        .manage(application_state_guard)
        .invoke_handler(tauri::generate_handler![
            logln,
            new_window_session,
            new_snippet,
            validate_pipeline_connection,
            new_pipeline,
            check_pipeline_connector_capacity_full,
            get_id,
            get_pipeline_connector_uuids_from_pipeline,
            delete_pipeline,
            get_snippet_pipelines,
            get_pipeline_connector_uuids_from_snippet,
            delete_snippet,
            spawn_initialize_snippet_directory_and_workspace,
            get_snippet_directory_details,
            update_snippet_parameter_value,
            spawn_run_snippets,
            save_project,
            update_snippet_position,
            get_directory_id_from_package_path,
            get_front_parameter_id_from_snippet_uuid_and_name,
            get_front_snippet_connector_id_from_snippet_uuid_and_name,
            open_project,
            get_workspace_details
        ])
        .run(tauri::generate_context!())
        .expect("error while starting tauri application");
}

#[tauri::command]
fn logln(text: &str) {
    println!("{text}");
}

//IMPORTAINT
//installed libpython3.10-dev

// plan for project saving:
// save rust state of snippets by: python path
//  save connections by their ids
//
// 1. just save the internal snippet manager as serialized
// 2. for each snippet, look up snippet id by the python path
//  make call on front end to create snippet
// 3. for each snippet pipeline
//  lookup connection similarly, then create pipeline again using front end call
//
// ideally, rust creates a "plan" of these actions, then sends as one as a return to the front end, which then executes it
//
// so higher level:
//  - rust creates plan with correct new snippet information
//  - front end executes it
//
// testing:
//  test as is
//  test when creating new snippet after saving but before next load, on start of program
//
// once this feature is enables, mainly being that we are treating the current canvas as the only saved state,
//  we can then work on creating the project directory, project names, etc
// Build plan:
// This is a rust structure that is based on a trait BuildRecreatePlan
//   contains
//   1. steps to call from front end to recreate
//      - actions: actions to recreate
//      - parameters/dependencies: values to be determined at runtime, with hints
//      - this is a ordered list
//   2. metadata on dependencies to resolve at runtime, and how to resolve them
//      - values from the parameters that tell which metadata to resolve to in the actions section
//      - the value to resolve with
//      - multiple sections for metadata based on the type of resolve, so there will be many types of resolvers
