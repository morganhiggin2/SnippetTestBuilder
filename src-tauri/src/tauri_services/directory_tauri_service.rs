use std::ops::DerefMut;

use crate::{core_services::io_service::FrontSnippetContent, state_management::MutexApplicationState};

#[tauri::command]
pub fn get_snippet_directory(application_state: tauri::State<MutexApplicationState>) -> Vec<FrontSnippetContent> {
    // get the state
    let state_guard = &mut application_state.0.lock().unwrap();
    let state = state_guard.deref_mut();

    //borrow split
    let seq_id_generator = &mut state.seq_id_generator;
    let ext_snippet_manager = &mut state.external_snippet_manager;
    let directory_manager = &mut state.directory_manager;

    return directory_manager.snippet_structure.file_structure_to_front_snippet_contents(seq_id_generator, ext_snippet_manager);
}