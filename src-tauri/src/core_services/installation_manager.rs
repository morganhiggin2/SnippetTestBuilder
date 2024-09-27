use reqwest;
use std::{
    env::home_dir,
    fs::{self, create_dir_all, read_dir},
    io::{Read as _, Write},
    path::PathBuf,
};

use super::concurrent_processes::get_runables_directory;

// The files needed for the base python runables
/// Responsible for installing the files upon launch in the right location on the computer if not already there
/// The files to install will be bundled with the launcher
const SNIPPET_CREATOR_FILE_CONTENTS: &str = include_str!("../../runables/snippet_creator.py");
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
    // create folder if it does not exist
    if !install_path.exists() {
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
    }

    //  create files within this folder if file does not eixst
    for child_file in folder.files {
        // file path
        let child_install_path = install_path.to_owned().join(child_file.name.to_owned());

        // only create the file if it does exist
        if !child_install_path.exists() {
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

/// Checks for any new versions of the standars snippets
/// and downloads if they don't exists, or updates if it exists.
/// This is done in parallel to program execution, and changes will be visible
/// in the directory manager, the next startup of the program, as to now slow down startup time
pub async fn fetch_new_snippets_zip() -> Result<(), String> {
    let runables_directory = get_runables_directory();

    // download metadata file, check for new versions
    let metadata_url =
        "https://www.snippettestbuilder.com/download/_standard_snippets/metadata.txt";
    let metadata_path = runables_directory.join(".metadata");

    download_file(metadata_url.to_string(), metadata_path.to_owned()).await?;

    // wether we are going to download the new version
    let mut download_new_version = false;

    // new version to be got from metadata file
    let mut new_version = String::new();

    // this scope allows the metadata file to be dropped after we are done using it
    {
        // examine version
        // open metadata file
        let mut metadata_file = match fs::File::open(metadata_path.to_owned()) {
            Ok(some) => some,
            Err(e) => {
                return Err(format!(
                    "Could not open metadata file at {}: {}",
                    metadata_path.to_string_lossy(),
                    e.to_string()
                ))
            }
        };

        // read file contents
        let mut metadata_file_contents = String::new();
        match metadata_file.read_to_string(&mut metadata_file_contents) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!(
                    "Could not read contents of metadata file {}: {}",
                    metadata_path.to_string_lossy(),
                    e.to_string()
                ))
            }
        };

        metadata_file_contents.pop();

        // get version (just going to be the only vonent in the file)
        new_version = metadata_file_contents;
    }

    let lock_file_path = runables_directory.join(".metadatalock");

    // if lock file does not exist
    if !lock_file_path.exists() {
        // no version was ever read, we are going to download the new version
        download_new_version = true;
    } else {
        // get lockfile version
        let mut current_version = String::new();

        {
            // read file contents
            let mut lock_file_contents = match fs::read(lock_file_path.to_owned()) {
                Ok(some) => some,
                Err(e) => {
                    return Err(format!(
                        "Could not read contents of lock file {}: {}",
                        lock_file_path.to_string_lossy(),
                        e.to_string()
                    ))
                }
            };

            // trim end of file character off end of file
            lock_file_contents.pop();

            // get version (just going to be the only vonent in the file)
            current_version = match String::from_utf8(lock_file_contents) {
                Ok(some) => some,
                Err(e) => {
                    return Err(format!(
                        "Could not convert lock file {} contents to UTF-8: {}",
                        lock_file_path.to_string_lossy(),
                        e.to_string()
                    ))
                }
            };
        }

        // compare versions
        let version_compare_result =
            compare_versions(new_version.to_owned(), current_version.to_owned())?;

        // if new version is not equal to current version
        if version_compare_result == 1 || version_compare_result == -1 {
            download_new_version = true;
        }
    }

    // if we are going to download the new version
    if download_new_version {
        let snippets_zip_path = runables_directory.join("snippets.zip");

        // Scope for snippets zip file
        {
            // create url for snippets zip from new version
            let snippets_zip_url = format!("https://www.snippettestbuilder.com/download/_standard_snippets/{}/standard_snippets.zip", new_version);

            download_file(snippets_zip_url.to_string(), snippets_zip_path.to_owned()).await?;
        }

        {
            // create lock file, truncating one if it exists
            let mut lock_file = match fs::File::create(lock_file_path.to_owned()) {
                Ok(file) => file,
                Err(e) => {
                    return Err(format!(
                        "Could not create version lock file at {}: {}",
                        lock_file_path.to_string_lossy(),
                        e.to_string()
                    ));
                }
            };

            // write new version to lock file
            match lock_file.write_all(new_version.as_bytes()) {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!(
                        "Could not write to version lock file {}: {}",
                        lock_file_path.to_string_lossy(),
                        e.to_string()
                    ));
                }
            }
        }
    }

    // delete metadata file
    {
        match fs::remove_file(metadata_path.to_owned()) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!(
                    "Could not remove metadata file at {}: {}",
                    metadata_path.to_string_lossy(),
                    e.to_string()
                ));
            }
        }
    }

    return Ok(());
}

/// Unpack zip file if it exists
pub async fn unpack_snippet_zip_if_exists() -> Result<(), String> {
    let runables_directory = get_runables_directory();
    let snippets_zip_path = runables_directory.join("snippets.zip");

    // if snippet zip exists
    if snippets_zip_path.exists() {
        {
            // unzip the file, overwriting existing contents in the process
            let snippets_zip_file = match fs::File::open(snippets_zip_path.to_owned()) {
                Ok(file) => file,
                Err(e) => {
                    return Err(format!(
                        "Could not open snippets zip file {}: {}",
                        snippets_zip_path.to_string_lossy(),
                        e.to_string()
                    ));
                }
            };

            // create the archive unziper
            let mut archive = match zip::ZipArchive::new(snippets_zip_file) {
                Ok(some) => some,
                Err(e) => {
                    return Err(format!(
                        "Could not create zip archive from file {}: {}",
                        snippets_zip_path.to_string_lossy(),
                        e.to_string()
                    ))
                }
            };

            // extract file to runnables location
            match archive.extract(runables_directory.to_owned()) {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!(
                        "Could not extract snippet zip file contents to {}: {}",
                        runables_directory.to_string_lossy(),
                        e.to_string()
                    ));
                }
            };
        }

        // delete snippets.zip file
        {
            match fs::remove_file(snippets_zip_path.to_owned()) {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!(
                        "Could not remove metadata file at {}: {}",
                        snippets_zip_path.to_string_lossy(),
                        e.to_string()
                    ));
                }
            }
        }
    }

    return Ok(());
}

/// check if one version is greater than the other
/// returns 1 if version a is greater than version b
/// returns 0 if version a is equal to version b
/// returns -1 if version a is less than version b
///
/// version format: "xx.xx.xx.xx"
/// where xx can be any whole number with no leading 0's of any size (no greater than the 64 bit ingeger limit)
///
/// # Arguments
///
/// * 'version_a' - version a
/// * 'version_b' - version b
fn compare_versions(version_a: String, version_b: String) -> Result<i8, String> {
    let version_a_parts: Vec<String> = version_a.split('.').map(|s| s.to_string()).collect();
    let version_b_parts: Vec<String> = version_b.split('.').map(|s| s.to_string()).collect();

    // convert each version part to an interget
    let version_a_numbers: Vec<i64> = version_a_parts
        .into_iter()
        .map(|s| match s.parse::<i64>() {
            Ok(some) => Ok(some),
            Err(e) => {
                return Err(format!(
                    "Could not convert string '{}' to i64 value: {}",
                    s,
                    e.to_string()
                ));
            }
        })
        .collect::<Result<Vec<i64>, String>>()?;
    let version_b_numbers: Vec<i64> = version_b_parts
        .into_iter()
        .map(|s| match s.parse::<i64>() {
            Ok(some) => Ok(some),
            Err(e) => {
                return Err(format!(
                    "Could not convert string '{}' to i64 value: {}",
                    s,
                    e.to_string()
                ));
            }
        })
        .collect::<Result<Vec<i64>, String>>()?;

    // if not the same length
    if version_a_numbers.len() != version_b_numbers.len() {
        return Err(format!("versions do not have the same number of version numbers, and hence is not valid to compare"));
    }

    for i in 0..version_a_numbers.len() {
        if version_a_numbers[i] > version_b_numbers[i] {
            return Ok(1);
        } else if version_a_numbers[i] < version_b_numbers[i] {
            return Ok(-1);
        }
    }

    return Ok(0);
}

/// Download a file from a url to the file_path destination
async fn download_file(url: String, file_path: PathBuf) -> Result<(), String> {
    // create request client
    let client = match reqwest::Client::builder().build() {
        Ok(some) => some,
        Err(e) => {
            return Err(format!(
                "Could not build reqwest builder: {}",
                e.to_string()
            ));
        }
    };

    let mut headers = reqwest::header::HeaderMap::new();
    headers.append("Host", "www.snippettestbuilder.com".parse().unwrap());
    headers.append("User-Agent", "SnippetTestBuilderApp".parse().unwrap());
    headers.append("Cache-Control", "no-cache".parse().unwrap());

    let request = client.request(reqwest::Method::GET, &url).headers(headers);

    let response = match request.send().await {
        Ok(some) => some,
        Err(e) => {
            return Err(format!(
                "Could not make request to url {}: {}",
                url,
                e.to_string()
            ));
        }
    };

    let body = match response.bytes().await {
        Ok(some) => some,
        Err(e) => {
            return Err(format!(
                "Failed to read response bytes from url request {}: {}",
                url,
                e.to_string()
            ))
        }
    };

    let mut file = match std::fs::File::create(file_path.to_owned()) {
        Ok(some) => some,
        Err(e) => {
            return Err(format!(
                "Failed to create file at path {}: {}",
                file_path.to_string_lossy(),
                e.to_string()
            ));
        }
    };

    // copy contents from the request to the file
    match std::io::copy(&mut body.as_ref(), &mut file) {
        Ok(_) => (),
        Err(e) => {
            return Err(format!(
                "Could not copy contents to file {}: {}",
                file_path.to_string_lossy(),
                e.to_string()
            ));
        }
    };

    return Ok(());
}

#[cfg(test)]
mod tests {
    use crate::core_services::installation_manager::compare_versions;

    #[test]
    fn test_compare_versions() {
        // versions of size one
        assert_eq!(compare_versions("14".to_string(), "0".to_string()), Ok(1));
        assert_eq!(compare_versions("2".to_string(), "2".to_string()), Ok(0));
        assert_eq!(compare_versions("0".to_string(), "8".to_string()), Ok(-1));

        // versions of size three
        assert_eq!(
            compare_versions("2.5.0".to_string(), "1.78.9".to_string()),
            Ok(1)
        );
        assert_eq!(
            compare_versions("0.0.1".to_string(), "0.0.1".to_string()),
            Ok(0)
        );
        assert_eq!(
            compare_versions("1.78.9".to_string(), "1.78.9".to_string()),
            Ok(0)
        );
        assert_eq!(
            compare_versions("1.24.2".to_string(), "1.78.9".to_string()),
            Ok(-1)
        );
        assert_eq!(
            compare_versions("0.24.2".to_string(), "1.78.9".to_string()),
            Ok(-1)
        );

        // empty versions
        matches!(compare_versions("".to_string(), "".to_string()), Err(_));

        // negitive version (invalid)
        matches!(
            compare_versions("-5.0.0".to_string(), "0.0.0".to_string()),
            Err(_)
        );

        // other invalid versions
        matches!(
            compare_versions("1.2.3".to_string(), "a.b.c".to_string()),
            Err(_)
        );
    }
}
