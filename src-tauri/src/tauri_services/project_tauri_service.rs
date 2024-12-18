use crate::{
    core_services::{
        concurrent_processes::get_projects_directory,
        project_service::{get_project_directory_location_from_name, Plan},
    },
    state_management::{
        external_snippet_manager::PackagePath, window_manager::WindowSession, ApplicationState,
        SharedApplicationState,
    },
    utils::sequential_id_generator::Uuid,
};
use std::{
    ops::DerefMut,
    sync::{Arc, MutexGuard},
};

///the service for commands between tauri and the front end
#[tauri::command]
pub fn save_project(
    application_state: tauri::State<SharedApplicationState>,
    window_session_uuid: Uuid,
    project_name: String,
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

    // remove project parent part from name
    let project_name = project_name.trim_start_matches("projects.").to_string();

    // get location to save project
    let projects_location = get_project_directory_location_from_name(project_name);

    // save project
    project_manager.save_project(external_snippet_manager, projects_location)?;

    return Ok(());
}
///the service for commands between tauri and the front end
#[tauri::command]
pub fn open_project(
    application_state: tauri::State<SharedApplicationState>,
    window_session_uuid: Uuid,
    project_id: String,
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

    // remove project parent part from name
    let project_name = project_id.trim_start_matches("projects.").to_string();

    // get location to save project from project name
    let project_location = get_project_directory_location_from_name(project_name);

    // try to get project build plan
    let project_build_plan = match project_manager.open_project(project_location) {
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

    // clear visual snippet service

    // clear snippet service

    // send plan to front end, load
    return Ok(project_build_plan);
}

#[tauri::command]
pub fn delete_project(
    application_state: tauri::State<SharedApplicationState>,
    window_session_uuid: Uuid,
    project_id: String,
) -> Result<(), String> {
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

    // remove project parent part from name
    let project_name = project_id.trim_start_matches("projects.").to_string();

    // delete project
    // NOTE this function surpresses errors
    project_manager.delete_project(project_name);

    return Ok(());
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

    // get directory entry uuid from path
    let directory_uuid = match directory_manager.find_directory_entry(package_path) {
        Some(directory_entry) => directory_entry.get_uuid(),
        None => {
            return (Err(format!(
                "Could not find directory uuid for {}, must not exist anymore",
                snippet_path
            )))
        }
    };

    // find front directory uuid
    let directory_front_uuid = match directory_manager
        .visual_component_manager
        .find_directory_front_uuid(&directory_uuid)
    {
        Some(uuid) => uuid,
        None => {
            return Err("Could not find directory front uuid".to_string());
        }
    };

    return Ok(directory_front_uuid);
}

// create pipeline:
// given a front snippet id and a snippet component name, give me the front snippet component id
#[tauri::command]
pub fn get_front_snippet_connector_id_from_snippet_uuid_and_name(
    application_state: tauri::State<SharedApplicationState>,
    window_session_uuid: Uuid,
    front_snippet_id: Uuid,
    snippet_connector_name: &str,
) -> Result<Uuid, String> {
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
    let project_manager = &window_session.project_manager;

    let visual_snippet_component_manager = &project_manager.visual_component_manager;
    let snippet_manager = &project_manager.snippet_manager;

    // get snippet id from front snippet id
    let snippet_uuid = match visual_snippet_component_manager.find_snippet_uuid(&front_snippet_id) {
        Some(uuid) => uuid,
        None => {
            return Err("Could not find snippet uuid for the given front snippet id".to_string());
        }
    };

    // get snippet
    let snippet = match snippet_manager.find_snippet(&snippet_uuid) {
        Some(s) => s,
        None => return Err("Could not find snippet for the given snippet uuid".to_string()),
    };

    // find the snippet connector by name
    let snippet_connector =
        match snippet.find_pipeline_connector_from_name(snippet_connector_name.to_string()) {
            Some(connector) => connector,
            None => {
                return Err(format!(
                    "Could not find pipeline connector for the given name {}",
                    snippet_connector_name.to_string()
                ));
            }
        };

    // get front snippet connector uuid from snippet connector uuid
    let front_snippet_connector_uuid = match visual_snippet_component_manager
        .find_pipeline_connector_front_uuid(&snippet_connector.get_uuid())
    {
        Some(uuid) => uuid,
        None => {
            return Err(
                "Could not find front snippet connector uuid for the given snippet connector"
                    .to_string(),
            );
        }
    };

    // NOTE: then way this should work is it gets the id of the connector from the external snippet manager, then
    // gets the ids up the chain.

    return Ok(front_snippet_connector_uuid);
}

// TODO parameter
// create pipeline:
// given a front snippet id and a snippet component name, give me the front snippet component id
#[tauri::command]
pub fn get_front_parameter_id_from_snippet_uuid_and_name(
    application_state: tauri::State<SharedApplicationState>,
    window_session_uuid: Uuid,
    front_snippet_id: Uuid,
    parameter_name: &str,
) -> Result<Uuid, String> {
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
    let project_manager = &window_session.project_manager;

    let visual_snippet_component_manager = &project_manager.visual_component_manager;
    let snippet_manager = &project_manager.snippet_manager;

    // get snippet id from front snippet id
    let snippet_uuid = match visual_snippet_component_manager.find_snippet_uuid(&front_snippet_id) {
        Some(uuid) => uuid,
        None => {
            return Err("Could not find snippet uuid for the given front snippet id".to_string());
        }
    };

    // get snippet
    let snippet = match snippet_manager.find_snippet(&snippet_uuid) {
        Some(s) => s,
        None => return Err("Could not find snippet for the given snippet uuid".to_string()),
    };

    // find the nsippet connector by name
    let parameter = match snippet.find_parameter_from_name(parameter_name.to_string()) {
        Some(connector) => connector,
        None => {
            return Err(format!(
                "Could not find parameter name for the given name {}",
                parameter_name.to_string()
            ));
        }
    };

    // get front snippet connector uuid from snippet connector uuid
    let front_snippet_parameter_uuid = match visual_snippet_component_manager
        .find_parameter_front_uuid(&parameter.get_uuid())
    {
        Some(uuid) => uuid,
        None => {
            return Err(
                "Could not find front parameter uuid for the given snippet connector".to_string(),
            );
        }
    };

    // NOTE: then way this should work is it gets the id of the connector from the external snippet manager, then
    // gets the ids up the chain.

    return Ok(front_snippet_parameter_uuid);
}
