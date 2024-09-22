use std::{fs::{self, read_dir}, path::PathBuf};

/// Responsible for installing the files upon launch in the right location on the computer if not already there
/// The files to install will be bundled with the launcher

pub fn install_runables(app: tauri::AppHandle) {
    // base directory 
    let install_path = PathBuf::new("~/usr/".to_string());

    // get bundles resources path
    let source_path = app.path_resolver()
      .resolve_resource("runables/*")
      .expect("failed to resolve resource");

    let relative_path = "";

    match put_runable_crawler(source_path, install_path, relative_path);

    // for each resource in runables

        // check if it exists

        // if it does not exist
            // base directory = path.pop()
            // ensure that base directories exist, if not create

            // std::fs::create_dir_all

            // install the file 
} 

/*
Arguments:

* 'source_base_path' - source base path
* 'install_base_path' - install base path
* 'relative_path' - relative path
*/
fn put_runable_crawler(source_base_path: PathBuf, install_base_path: PathBuf, relative_path: PathBuf) -> Result<(), String> {
    // get the full paths 
    let source_path = source_base_path.join(relative_path.to_owned());
    let install_path = install_base_path.join(relative_path.to_owned());

    // if the install path is a directory and the source path does not exist
    if install_path.is_dir() && !source_path.exists() {
        // create the directory if not exist
        match fs::create_dir(install_path.to_owned()) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!("Could not create directory at location {}: {}", install_path.to_owned().to_string_lossy(), e.to_string()));
            }
        };

        // walk child directories
        let directory_entries = match read_dir(install_path.to_owned()) {
            Ok(some) => some,
            Err(e) => {
                return Err(format!("Could not read directory entries in path {}: {}", install_path.to_string_lossy(), e.to_string()));
            }
        };

        for dir_entry_result in directory_entries {
            let dir_entry = match dir_entry_result {
                Ok(some) => some,
                Err(e) => {
                    return Err(format!("Could not get directory entry of parent path {}: {}", install_path.to_string_lossy(), e.to_string()));
                }
            };

            // get path from directory entry
            let child_path = dir_entry.path();
            
            // get relative directory
            let new_relative_path = relative_path.to_owned().join(child_path.file_name().unwrap());

            // make child call
            put_runable_crawler(source_base_path.to_owned(), install_base_path.to_owned(), new_relative_path)?;
        } 
    }
    // if the install path is a file and the source path does not exist
    if install_path.is_dir() && !source_path.exists() {
        // copy the file
        //create the directory if not exist
        match fs::copy(source_path.to_owned(), install_path.to_owned()) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!("Could not create directory at location {}: {}", install_path.to_string_lossy(), e.to_string()));
            }
        };
    }


    return Ok(());
        
    // if the install path is a file, install file if it does not exist 

}