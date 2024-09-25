use std::{
    env::home_dir,
    fs::{self, create_dir_all, read_dir},
    path::PathBuf,
};

use super::concurrent_processes::get_runables_directory;

// The files needed for the base python runables
const snippet_creator_file_contents: &str = include_str!("../../runables/snippet_creator.py");
const snippet_runner_file_contents: &str = include_str!("../../runables/snippet_runner.py");

/// Responsible for installing the files upon launch in the right location on the computer if not already there
/// The files to install will be bundled with the launcher

// structs for the visual directory
struct VirtualFolder {
    name: &'static str,
    folders: &'static [VirtualFolder],
    files: &'static [VirtualFile],
}

struct VirtualFile {
    name: &'static str,
    contents: &'static str,
}

// create static virtual file directory
const virutal_directory: VirtualFolder = VirtualFolder {
    name: &"runables",
    files: &[
        VirtualFile {
            name: "snippet_runner.py",
            contents: snippet_runner_file_contents,
        },
        VirtualFile {
            name: "snippet_creator.py",
            contents: snippet_creator_file_contents,
        },
    ],
    folders: &[],
};

pub fn install_runables(app: &mut tauri::App) {
    let install_path = get_runables_directory();

    // create base directory if not exists
    create_dir_all(install_path.to_owned()).unwrap();

    // create visual direction in install location
    put_runable_crawler(source_path, install_path).unwrap();
}

/// helper method to install files from the virual directory
///
/// # Arguments
/// * 'folder' - virtual folder
/// * 'install_path' - install base path
fn install_runable_crawer(folder: &VirtualFolder, install_path: PathBuf) -> Result<(), String> {
    // create the folder
    match fs::create_dir(install_path.to_owned()) {
        Ok(_) => (),
        Err(e) => {
            return Err(format!(
                "Could not create directory at location {}: {}",
                install_path.to_owned().to_string_lossy(),
                e.to_string()
            ));
        }
    };

    //  create files within this folder
    for child_file in folder.files {
        // file path
        let child_install_path = install_path.to_owned().join(child_file.name.to_owned());

        // create file
        let mut file: fs::File = match fs::File::create(child_install_path) {
            Ok(some) => some,
            Err(e) => {
                return Err(format!(
                    "Could not create file {}: {}",
                    child_install_path.to_string_lossy(),
                    e.to_string()
                ));
            }
        };

        // write contents
        match file.write_all(child_file.contents) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!(
                    "Could not write to file {}: {}",
                    child_install_path.to_string_lossy(),
                    e.to_string()
                ));
            }
        }
    }

    // for each child folder folder
    for child_folder in folder.folders {
        // set new installation path
        let child_install_path = install_path.to_owned().join(child_folder.name.to_owned());

        // make recursive call
        install_runable_crawer(child_folder, child_install_path)?;
    }

    return Ok(());
}

/// helper method to install files from source directory to installation directory if they do not already exist
///
/// # Arguments:
///
/// * 'source_path' - source path
/// * 'install_path' - install path
/// * 'relative_path' - relative path
fn put_runable_crawler(source_path: PathBuf, install_path: PathBuf) -> Result<(), String> {
    // if the install path is a directory and the source path does not exist
    if source_path.is_dir() {
        // if it does not exist already
        if !install_path.exists() {
            // create the directory if not exist
            match fs::create_dir(install_path.to_owned()) {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!(
                        "Could not create directory at location {}: {}",
                        install_path.to_owned().to_string_lossy(),
                        e.to_string()
                    ));
                }
            };
        }

        // walk child directories
        let directory_entries = match read_dir(source_path.to_owned()) {
            Ok(some) => some,
            Err(e) => {
                return Err(format!(
                    "Could not read directory entries in path {}: {}",
                    source_path.to_string_lossy(),
                    e.to_string()
                ));
            }
        };

        for dir_entry_result in directory_entries {
            let dir_entry = match dir_entry_result {
                Ok(some) => some,
                Err(e) => {
                    return Err(format!(
                        "Could not get directory entry of parent path {}: {}",
                        install_path.to_string_lossy(),
                        e.to_string()
                    ));
                }
            };

            // get path from directory entry
            let child_source_path = dir_entry.path();

            // get install path
            let child_install_path = install_path
                .to_owned()
                .join(child_source_path.file_name().unwrap());

            // make child call
            put_runable_crawler(child_source_path, child_install_path)?;
        }
    }
    // if the source path is a file and the install path does not exist
    else if !source_path.is_dir() && !install_path.exists() {
        // copy the file
        //create the directory if not exist
        match fs::copy(source_path.to_owned(), install_path.to_owned()) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!(
                    "Could not create directory at location {}: {}",
                    install_path.to_string_lossy(),
                    e.to_string()
                ));
            }
        };
    }

    return Ok(());

    // if the install path is a file, install file if it does not exist
}
