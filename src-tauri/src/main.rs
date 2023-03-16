// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use crate::state_management::{MutexApplicationState, ApplicationState};
use crate::tauri_services::window_session_tauri_service::{new_window_session};

pub mod state_management;
pub mod utils;
pub mod core_components;
pub mod tauri_services;

fn main() {
    tauri::Builder::default()
        .manage(MutexApplicationState (Mutex::new(ApplicationState::default())))
        .invoke_handler(tauri::generate_handler![logln])
        .invoke_handler(tauri::generate_handler![new_window_session])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn logln(text: &str) {
    println!("{text}");
}