use std::{
    env,
    ops::DerefMut,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use tauri::Manager;

use crate::{
    python_libraries::python_run_module::InitializedPythonSnippetRunnerBuilder,
    state_management::ApplicationState,
};

use super::{runtime_logging_service::LoggingStreamInstance, workspace_manager::WorkspaceManager};

/// This event spawns the initalize directory and workspace event, returning the event id and the log file id.
/// This will emit the event id to the front id  when the process is complete
pub async fn spawn_initialize_directory_event(
    application_state: Arc<Mutex<ApplicationState>>,
    logging_stream_instance: &mut LoggingStreamInstance,
) {
    // lock the application state
    let mut state_guard = application_state.lock().unwrap();
    let state = state_guard.deref_mut();

    let sequential_id_generator = &mut state.sequential_id_generator;
    let external_snippet_manager = &mut state.external_snippet_manager;
    let directory_manager = &mut state.directory_manager;

    // Initialize directory of snippets
    directory_manager
        .initialize(&"snippets/root".to_string(), sequential_id_generator)
        .unwrap();

    // create external snippets from directory manager
    let result = external_snippet_manager
        .create_external_snippets_from_directory(directory_manager, sequential_id_generator);

    // TODO very basic logging, will expand in future
    if let Err(e) = result {
        logging_stream_instance.append_log(format!("Failed to initialize snippet: {}", e));
    } else {
        logging_stream_instance
            .append_log(format!("Finished successfully intializing all snippets"));
    }
}

pub async fn spawn_initialize_workspace_event(application_state: Arc<Mutex<ApplicationState>>) {
    // lock the application state
    let mut state_guard = application_state.lock().unwrap();
    let state = state_guard.deref_mut();

    // Initialize directory of snippets
    let new_workspace_manager = WorkspaceManager::initialize().unwrap();

    // swap
    state.workspace_manager = new_workspace_manager;
}

pub async fn spawn_run_snippets_event(
    build_state: InitializedPythonSnippetRunnerBuilder,
    mut logging_stream_instance: LoggingStreamInstance,
) {
    // run the build state
    match build_state.run(&mut logging_stream_instance) {
        Ok(_) => {
            logging_stream_instance
                .append_log(format!("Finished successfully running all snippets"));
        }
        Err(e) => {
            // TODO remove
            logging_stream_instance.append_log(e);
        }
    };

    // close the log
    let app_handle = logging_stream_instance.close_log();

    // emit event back to front end
    app_handle.emit_all("snippets_ran", "".to_string()).unwrap();
}

pub fn get_working_directory() -> PathBuf {
    // default method of getting directory
    let mut working_directory = env::current_dir().unwrap();

    // if we are on mac
    if !cfg!(debug_assertions) && cfg!(target_os = "macos") {
        // get current executable location
        working_directory = env::current_exe().unwrap();
        // remove the executable name from the path to get the base folder
        working_directory.pop();
    }

    return working_directory;
}

// get runables directory
pub fn get_runables_directory() -> PathBuf {
    // if we are in test mode
    if cfg!(test) {
        return get_working_directory().join("runables");
    } else {
        // base directory
        return directories::UserDirs::new()
            .unwrap()
            .home_dir()
            .join("SnippetTestBuilder/runables");
    }
}

// get project save directory
pub fn get_projects_directory() -> PathBuf {
    // if we are in test mode
    if cfg!(test) {
        return get_working_directory().join("projects");
    } else {
        // base directory
        return directories::UserDirs::new()
            .unwrap()
            .home_dir()
            .join("SnippetTestBuilder/projects");
    }
}
