use std::{
    env::home_dir,
    fs::{self, create_dir_all, read_dir},
    io::Write,
    path::PathBuf,
};

use super::concurrent_processes::get_runables_directory;

// The files needed for the base python runables
/// Responsible for installing the files upon launch in the right location on the computer if not already there
/// The files to install will be bundled with the launcherconst SNIPPET_CREATOR_FILE_CONTENTS: &str = include_str!("../../runables/snippet_creator.py");
const SNIPPET_RUNNER_FILE_CONTENTS: &str = include_str!("../../runables/snippet_runner.py");

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
const VIRTUAL_DIRECTORY: VirtualFolder = VirtualFolder {
    name: &"runables",
    files: &[
        VirtualFile {
            name: "snippet_runner.py",
            contents: SNIPPET_RUNNER_FILE_CONTENTS,
        },
        VirtualFile {
            name: "snippet_creator.py",
            contents: SNIPPET_CREATOR_FILE_CONTENTS,
        },
    ],
    folders: &[VirtualFolder {
        name: "snippets",
        files: &[],
        folders: &[VirtualFolder {
            name: "root",
            files: &[],
            folders: &[],
        }],
    }],
};

pub fn install_runables() {
    let install_path = get_runables_directory();

    // create base directory if not exists
    create_dir_all(install_path.to_owned()).unwrap();

    // create visual direction in install location
    install_runable_crawer(&VIRTUAL_DIRECTORY, install_path).unwrap();
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
        let mut file = match fs::File::create(child_install_path.to_owned()) {
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
        match file.write_all(child_file.contents.as_bytes()) {
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
