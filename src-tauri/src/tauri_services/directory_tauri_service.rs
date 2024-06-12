use std::ops::DerefMut;

use crate::{core_services::front_directory_component_manager::FrontExternalSnippetContent, state_management::MutexApplicationState};

/// get the snippet directory in it's entirety, and it's information
#[tauri::command]
pub fn get_snippet_directory(application_state_guard: tauri::State<MutexApplicationState>) -> Vec<FrontExternalSnippetContent> {
    // get the state
    let state_guard = &mut application_state_guard.0.lock().unwrap();
    let state = state_guard.deref_mut();

    //borrow split
    let seq_id_generator = &mut state.seq_id_generator;
    let ext_snippet_manager = &mut state.external_snippet_manager;
    let directory_manager = &mut state.directory_manager;
    let visual_directory_component_manager = &mut directory_manager.visual_component_manager;

    //create front snippet containers and add to virtual manager
    return directory_manager.snippet_structure.file_structure_to_front_snippet_contents(visual_directory_component_manager, seq_id_generator, ext_snippet_manager);
}