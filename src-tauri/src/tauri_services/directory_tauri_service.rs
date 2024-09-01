use std::{ops::DerefMut, sync::{Arc, MutexGuard}};


use crate::{core_services::{concurrent_processes::spawn_initialize_directory_event, visual_directory_component_manager::FrontDirectoryContent}, state_management::{ApplicationState, SharedApplicationState}, utils::sequential_id_generator::Uuid};
#[tauri::command] 
pub fn get_snippet_directory_details(application_state: tauri::State<SharedApplicationState>) -> Vec<FrontDirectoryContent> {
    // get the state
    let mut state_guard: MutexGuard<ApplicationState> = application_state.0.lock().unwrap();
    let state = state_guard.deref_mut();

    let sequential_id_generator = &mut state.sequential_id_generator;
    let directory_manager = &mut state.directory_manager;

    // get front directory content
    return directory_manager.get_as_front(sequential_id_generator);
}

/// spawn initialize snippet directory, returning log stream id
#[tauri::command]
pub fn spawn_initialize_snippet_directory(application_state: tauri::State<SharedApplicationState>, app_handle: tauri::AppHandle, window_session_uuid: Uuid) -> u32 {
    // get the state
    let mut state_guard: MutexGuard<ApplicationState> = application_state.0.lock().unwrap();
    let state = state_guard.deref_mut();

    // create log file and stream from window uuid
    // that way the log instance is specific to the window uuid
    let logging_instance = state.logging_manager.create_new_stream(app_handle, window_session_uuid).unwrap();
    let stream_i = logging_instance.get_stream_i();

    // get shared reference to state 
    // note this is a custom clone implementation utilizing on arc::clone
    let application_state_ref : SharedApplicationState = SharedApplicationState(Arc::clone(&application_state.0));

    // spawn process, passing ownership of shared application state
    tauri::async_runtime::spawn(async move {
        spawn_initialize_directory_event(application_state_ref.0, logging_instance).await;     
    });

    return stream_i;
}

/*
/// get the snippet directory in it's entirety, and it's information
#[tauri::command]
pub fn get_snippet_directory(application_state_guard: tauri::State<MutexApplicationState>) -> Vec<FrontDirectoryContent> {
    // get the state
    let state_guard = &mut application_state_guard.0.lock().unwrap();
    let state = state_guard.deref_mut();

    //borrow split
    let sequential_id_generator = &mut state.sequential_id_generator;
    let directory_manager = &mut state.directory_manager;
    let visual_directory_component_manager = &mut directory_manager.visual_component_manager;

    //create front snippet containers and add to virtual manager
    return visual_directory_component_manager.get_directory_as_front(&directory_manager.snippet_directory, sequential_id_generator)
} */