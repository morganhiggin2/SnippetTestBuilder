use std::{ops::DerefMut, sync::{Arc, MutexGuard}};

use tauri::Manager;

use crate::{core_services::{concurrent_processes::spawn_initialize_directory_event, directory_manager::{self, DirectoryManager}, visual_directory_component_manager::FrontDirectoryContent}, state_management::{external_snippet_manager::ExternalSnippetManager, ApplicationState, SharedApplicationState}};
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

#[tauri::command]
pub fn spawn_initialize_snippet_directory(application_state: tauri::State<SharedApplicationState>, app_handle: tauri::AppHandle) {
    // get the state
    let mut state_guard: MutexGuard<ApplicationState> = application_state.0.lock().unwrap();
    let state = state_guard.deref_mut();

    // create log file, get stream id
    let logging_instance = state.logging_manager.create_new_stream().unwrap();

    // get shared reference to state 
    // note this is a custom clone implementation utilizing on arc::clone
    let application_state_ref : SharedApplicationState = SharedApplicationState(Arc::clone(&application_state.0));

    // spawn process, passing ownership of shared application state
    tauri::async_runtime::spawn(async move {
        spawn_initialize_directory_event(application_state_ref.0, logging_instance, app_handle).await;     
    });
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