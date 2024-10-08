// Create correct log location for purpose
// buffer, then append to log location

use std::{borrow::BorrowMut, sync::{Arc, Mutex}};

use tauri::Manager;

use crate::utils::sequential_id_generator::Uuid;

static BASE_LOG_PATH: &str = "/logging";

//TODO global list of taken streaming numbers, can be added to and taken away from
//Held in global tauri state
/*pub struct LoggingStreamManager (
    Arc::<Mutex::<LoggingStreamCoordinator>>
);

struct LoggingStreamCoordinator {
    active_streams: HashSet<u32> 
}

pub struct LoggingStreamInstance {
    stream_i: u32,
    file: std::fs::File,
    file_path: PathBuf,
    logging_streams: Arc<Mutex<LoggingStreamCoordinator>>
}

impl Default for LoggingStreamManager {
    fn default() -> Self {
        return LoggingStreamManager(Arc::new(Mutex::new(LoggingStreamCoordinator::default())));
    }
}

impl Default for LoggingStreamCoordinator {
    fn default() -> Self {
        Self { active_streams: Default::default() }
    }
}

impl LoggingStreamManager {
    pub fn create_new_stream(&mut self) -> Result<LoggingStreamInstance, String> {
        // get logging stream coordinator
        let mut logging_stream_coordinator_lock = self.0.lock().unwrap();
        let logging_stream_coordinator = logging_stream_coordinator_lock.borrow_mut();

        // impose limit until we implement multiple processes on the front end
        if logging_stream_coordinator.active_streams.len() > 1 {
            return Err("Due to pending parallel workflow processing, parallel log streaming will not be allowed".to_string());
        }

        // Start from 1, finding first stream index that is not already taken
        let range = Range::<u32> {start: 1, end: u32::max_value()};

        let mut stream_i: u32 = 0;

        // Log stream will always start at 1, so 0 will be an invalid stream id 
        for i in range.into_iter() {
            // Once we find an available one
            if !logging_stream_coordinator.active_streams.contains(&i) {
                stream_i = i;
            }
        }

        // All streams are taken, panic as this is an unforseen case
        // Later implementations can wait for one to become available
        if stream_i == 0 {
            panic!("All active streams are taken");
        }

        // create log file
        let (file, file_path) = Self::initialize_file_service(stream_i)?;

        // create log stream instance
        let logging_stream_instance = LoggingStreamInstance::new(Arc::clone(&self.0), stream_i, file, file_path)?; 

        // Add stream id to active streams after successful creation 
        logging_stream_coordinator.active_streams.insert(stream_i);

        return Ok(logging_stream_instance);

    }

    /// initalize log file
    fn initialize_file_service(stream_i: u32) -> Result<(std::fs::File, PathBuf), String>{
        //get current working directory
        let current_working_directory = match env::current_dir() {
            Ok(result) => result.as_path().to_owned(),
            Err(e) => {
                return Err(e.to_string());
            }
        };
        // get log file path
        let log_file_name = stream_i.to_string() + ".log";
        let log_file_path = current_working_directory.join(BASE_LOG_PATH).join(log_file_name);

        // Get rid of existing file if exists
        // this scenario can occur if the program crashed in the middle of an operation 
        if log_file_path.exists() {
            std::fs::remove_file(log_file_path.to_owned());
        }

        // Create new empty file in append mode, get fd, store
        let file = match std::fs::OpenOptions::new().append(true).open(log_file_path.to_owned()) {
            Ok(file) => file,
            Err(e) => {
                return Err(format!("Was not able to create log file for stream {}:{}", stream_i, e.to_string()));
            },
        };

        return Ok((file, log_file_path));
    }
}

impl LoggingStreamCoordinator {
    // Remove stream, irregardless if stream already exists or not
    pub fn done_with_stream(&mut self, stream_i: u32) {
        // remove stream from active streams
        self.active_streams.remove(&stream_i);
    }
}

// As long as the runtime service is alive, the file descriptor will be held
impl LoggingStreamInstance {
    /// Creates a new runtime logging service instance 
    pub fn new(logging_streams: Arc<Mutex<LoggingStreamCoordinator>>, stream_i: u32, file: std::fs::File, file_path: PathBuf) -> Result<Self, String> {
        // create new runtime logging service
        let service = LoggingStreamInstance {
            stream_i: stream_i,
            file: file,
            file_path: file_path,
            logging_streams: logging_streams
        };

        return Ok(service);
    }

    pub fn get_stream_i(&self) -> u32 {
        return self.stream_i;
    }


    // append log to file
    pub fn append_log(&mut self, log: String) {
        // append new line
        let contents = log + "\n"; 
        // append contents to file
        self.file.write(contents.as_bytes());
    }
}

impl Drop for LoggingStreamInstance {
    /// overwrite existing drop method to remove itself from the logging stream coordinator, and to remove the existing file
    fn drop(&mut self) {
        {
            // get logging stream coordinator
            let mut logging_stream_coordinator_lock = self.logging_streams.lock().unwrap();
            let logging_stream_coordinator = logging_stream_coordinator_lock.borrow_mut();

            // remove stream from stream coordinator
            logging_stream_coordinator.done_with_stream(self.stream_i);

            // get file path
            let file_path = self.file_path.to_owned();

            // attempt to remove file
            // if the file was moved, this will cause the program to crash
            // ignore error
            std::fs::remove_file(file_path);
        }

        // drop self
        drop(self);
    }
}*/

pub struct LoggingStreamManager (
    Arc::<Mutex::<LoggingStreamCoordinator>>
);

//TODO add uuids
// OR remove uuids from other things like this where it does not matter

struct LoggingStreamCoordinator {
    active_stream: Option<u32> 
}

pub struct LoggingStreamInstance {
    stream_i: u32,
    app_handle: tauri::AppHandle,
    logging_stream_coordinator: Arc<Mutex<LoggingStreamCoordinator>>
}

impl Default for LoggingStreamManager {
    fn default() -> Self {
        return LoggingStreamManager(Arc::new(Mutex::new(LoggingStreamCoordinator::default())));
    }
}

impl Default for LoggingStreamCoordinator {
    fn default() -> Self {
        Self { active_stream: Default::default() }
    }
}

impl LoggingStreamManager {
    /// Create a new stream.
    /// returns error if stream could not be created successfully
    pub fn create_new_stream(&mut self, app_handle: tauri::AppHandle, window_session_uuid: Uuid) -> Result<LoggingStreamInstance, String> {
        // get logging stream coordinator
        let mut logging_stream_coordinator_lock = self.0.lock().unwrap();
        let logging_stream_coordinator = logging_stream_coordinator_lock.borrow_mut();

        // impose limit until we implement multiple processes on the front end
        if let Some(_) = logging_stream_coordinator.active_stream {
            return Err("Due to pending parallel workflow processing, parallel log streaming will not be allowed".to_string());
        }

        let stream_i = window_session_uuid; 

        // create log stream instance
        let logging_stream_instance = LoggingStreamInstance::new(Arc::clone(&self.0), stream_i, app_handle)?; 

        // Add stream id to active streams after successful creation 
        logging_stream_coordinator.active_stream = Some(stream_i);

        return Ok(logging_stream_instance);
    }
}

impl LoggingStreamCoordinator {
    // Remove stream, irregardless if stream already exists or not
    pub fn done_with_stream(&mut self) {
        // remove stream from active stream
        self.active_stream = None;
    }
}

// As long as the runtime service is alive, the file descriptor will be held
impl LoggingStreamInstance {
    /// Creates a new runtime logging service instance 
    fn new(logging_streams: Arc<Mutex<LoggingStreamCoordinator>>, stream_i: u32, app_handle: tauri::AppHandle) -> Result<Self, String> {
        // create new runtime logging service
        let service = LoggingStreamInstance {
            stream_i: stream_i,
            app_handle: app_handle,
            logging_stream_coordinator: logging_streams
        };

        return Ok(service);
    }

    pub fn get_stream_i(&self) -> u32 {
        return self.stream_i;
    }

    /// append log
    pub fn append_log(&mut self, log: String) {
        // emit name
        let event_name = format!("log_{}", self.stream_i);
        // append new line
        let contents = log + "\n"; 
        // emit log
        self.app_handle.emit_all(&event_name, contents).unwrap();
    }

    /// Close the log
    pub fn close_log(self) -> tauri::AppHandle{
        // emit name
        let event_name = format!("log_{}", self.stream_i);
        // an empty log will be a closing signal
        // this is safe because each append log has at least the new line character
        let contents = ""; 
        // emit log
        self.app_handle.emit_all(&event_name, contents).unwrap();

        {
            // get logging stream coordinator
            let mut logging_stream_coordinator_lock = self.logging_stream_coordinator.lock().unwrap();
            let logging_stream_coordinator = logging_stream_coordinator_lock.borrow_mut();

            // close stream from stream coordinator
            logging_stream_coordinator.done_with_stream();
        }

        return self.app_handle;
    }
}

/*
#[cfg(test)]
mod tests {
    use tauri::Manager;

    #[test]
    fn test_log_streaming() {
        // create tauri app handle
        // create logging stream coordinator

        // create logging stream instance

        // append to log on this instance

        // try to create another logging stream instance, fail

        // drop logging stream instance

        // create another logging stream instance successfully
    }
}
*/