use std::{borrow::{Borrow, BorrowMut}, ops::DerefMut, sync::{Arc, Mutex}};

use tauri::Manager;

use crate::{state_management::{external_snippet_manager, ApplicationState}, utils::sequential_id_generator};

use super::{directory_manager, runtime_logging_service::LoggingStreamInstance};

/// This event spawns the initalize event, returning the event id and the log file id.
/// This will emit the event id to the front id  when the process is complete
pub async fn spawn_initialize_directory_event(application_state: Arc::<Mutex::<ApplicationState>>, mut logging_stream_instance: LoggingStreamInstance) {
    print!("here in process");
    // lock the application state
    let mut state_guard = application_state.lock().unwrap();
    let state = state_guard.deref_mut();

    let sequential_id_generator = &mut state.sequential_id_generator;
    let external_snippet_manager = &mut state.external_snippet_manager;   
    let directory_manager = &mut state.directory_manager;

    //TODO reinitialize directory if already initialized

    // Initialize directory of snippets
    directory_manager.initialize(&"../data/snippets/root".to_string(), sequential_id_generator).unwrap();

    // create external snippets from directory manager
    let result = external_snippet_manager.create_external_snippets_from_directory(directory_manager, sequential_id_generator);

    // TODO very basic logging, will expand in future
    if let Err(e) = result {
        logging_stream_instance.append_log(format!("Failed to initialize snippet: {}", e));
    }
    else {
        logging_stream_instance.append_log(format!("Finished successfully intializing all snippets"));
    }
    
    // close the log
    let app_handle = logging_stream_instance.close_log();

    // emit event back to front end
    app_handle.emit_all("directory_initialized", "".to_string()).unwrap(); 

    print!("done with process");
}