// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::DerefMut;
use std::sync::Mutex;

use crate::state_management::{MutexApplicationState, ApplicationState};
use crate::tauri_services::directory_tauri_service::get_snippet_directory;
use crate::tauri_services::snippet_tauri_service::new_snippet;
use crate::tauri_services::window_session_tauri_service::{new_window_session};

pub mod state_management;
pub mod utils;
pub mod core_components;
pub mod tauri_services;
pub mod core_services;

fn main() {
    //create application state
    let mut application_state_guard = MutexApplicationState::default();
    /*let b = &application_state_guard;

    let mut guard = b.lock().unwrap();
    let mut obj = guard.deref_mut();*/

    {
        let mut guard = application_state_guard.0.lock().unwrap();
        let app_ref = guard.deref_mut();

        //let mut application_state = (&application_state_guard).lock().unwrap().deref_mut();

        //call init for utils and services first

        //call init for state management system
        ApplicationState::init(app_ref);
    }

    tauri::Builder::default()
        .manage(application_state_guard)
        .invoke_handler(tauri::generate_handler![logln, new_window_session, get_snippet_directory, new_snippet])
        .run(tauri::generate_context!())
        .expect("error while starting tauri application");
}

#[tauri::command]
fn logln(text: &str) {
    println!("{text}");
}