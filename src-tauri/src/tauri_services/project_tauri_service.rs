use crate::{
    core_services::{concurrent_processes::get_projects_directory, project_service::Plan},
    state_management::{
        external_snippet_manager::{self, PackagePath},
        window_manager::WindowSession,
        SharedApplicationState,
    },
    utils::sequential_id_generator::Uuid,
};
use std::ops::DerefMut;

///the service for commands between tauri and the front end
#[tauri::command]
pub fn save_project(
    application_state: tauri::State<SharedApplicationState>,
    window_session_uuid: Uuid,
) -> Result<(), String> {
    // get the state
    let state_guard = &mut application_state.0.lock().unwrap();
    let state = &mut state_guard.deref_mut();

    //borrow split
    let window_manager = &mut state.window_manager;
    let external_snippet_manager = &mut state.external_snippet_manager;

    //find window session
    let window_session: &mut WindowSession =
        match window_manager.find_window_session_mut(window_session_uuid) {
            Some(result) => result,
            None => {
                return Err("window session could not be found".to_string());
            }
        };

    // borrow split
    let project_manager = &mut window_session.project_manager;

    // get location to save project
    let projects_location = get_projects_directory();

    // save project
    project_manager.save_project(external_snippet_manager, projects_location)?;

    return Ok(());
}
///the service for commands between tauri and the front end
#[tauri::command]
pub fn open_project(
    application_state: tauri::State<SharedApplicationState>,
    window_session_uuid: Uuid,
) -> Result<Plan, String> {
    // get the state
    let state_guard = &mut application_state.0.lock().unwrap();
    let state = &mut state_guard.deref_mut();

    //borrow split
    let window_manager = &mut state.window_manager;

    //find window session
    let window_session: &mut WindowSession =
        match window_manager.find_window_session_mut(window_session_uuid) {
            Some(result) => result,
            None => {
                return Err("window session could not be found".to_string());
            }
        };

    // borrow split
    let project_manager = &mut window_session.project_manager;

    // get location to save project
    let projects_location = get_projects_directory();

    // try to get project build plan
    let project_build_plan = match project_manager.open_project(projects_location) {
        Ok(some) => some,
        Err(e) => {
            // if cannot get plan, create default (empty) one
            println!(
                "Could not create project build plan: {}, resorting to empty build plan",
                e.to_string()
            );
            project_manager.get_default_plan()
        }
    };

    // send plan to front end, load
    return Ok(project_build_plan);
}

// create snippet:
// given an external snippet path, give me an external snippet id
#[tauri::command]
// TODO change this to get directory id
pub fn get_directory_id_from_package_path(
    application_state: tauri::State<SharedApplicationState>,
    snippet_path: &str,
) -> Result<Uuid, String> {
    // get the state
    let state_guard = &mut application_state.0.lock().unwrap();
    let state = &mut state_guard.deref_mut();

    // borrow split
    let directory_manager = &state.directory_manager;

    // get string path as package path
    let package_path: PackagePath = snippet_path.to_string().into();

    // get external snippet uuid from path
    return match directory_manager.find_directory_entry(package_path) {
        Some(directory_entry) => Ok(directory_entry.get_uuid()),
        None => Err(format!(
            "Could not find directory uuid for {}, must not exist anymore",
            snippet_path
        )),
    };
}

// create pipeline:
// given a front snippet id and a snippet component name, give me the front snippet component id

// TODO parameter
