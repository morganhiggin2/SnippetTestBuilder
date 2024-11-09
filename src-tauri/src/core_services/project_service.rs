use std::{
    io::{Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{
    core_components::snippet_manager::SnippetManager,
    state_management::{
        external_snippet_manager::{ExternalSnippetManager, PackagePath},
        visual_snippet_component_manager::VisualSnippetComponentManager,
    },
    utils::sequential_id_generator::Uuid,
};

use super::concurrent_processes::get_projects_directory;

// project manager
pub struct ProjectManager {
    pub snippet_manager: SnippetManager,
    pub visual_component_manager: VisualSnippetComponentManager,
}

impl ProjectManager {
    /// create a new window session
    pub fn new() -> Self {
        return ProjectManager {
            snippet_manager: SnippetManager::default(),
            visual_component_manager: VisualSnippetComponentManager::default(),
        };
    }
}

impl Default for ProjectManager {
    /// create a new window session
    fn default() -> Self {
        return ProjectManager {
            snippet_manager: SnippetManager::default(),
            visual_component_manager: VisualSnippetComponentManager::default(),
        };
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Plan {
    actions: PlanActions,
}

#[derive(Serialize, Deserialize, Default)]
struct PlanActions {
    build_snippet_actions: Vec<BuildSnippetAction>,
    build_snippet_pipeline_actions: Vec<BuildSnippetPipelineAction>,
    build_snippet_parameter_actions: Vec<BuildSnippetParameterAction>,
}

#[derive(Serialize, Deserialize)]
struct BuildSnippetAction {
    package_path: PackagePath,
    // this is to ensure we don't confused two snippets with the same package path
    original_uuid: Uuid,
    x_position: f64,
    y_position: f64,
}

#[derive(Serialize, Deserialize)]
struct BuildSnippetPipelineAction {
    // (from snippet path, from snippet connector name), (to snippet path, to snippet connector name)
    from_snippet_package_path: PackagePath,
    from_snippet_original_uuid: Uuid,
    from_snippet_connector_name: String,
    to_snippet_package_path: PackagePath,
    to_snippet_original_uuid: Uuid,
    to_snippet_connector_name: String,
}

#[derive(Serialize, Deserialize)]
struct BuildSnippetParameterAction {
    snippet_package_path: PackagePath,
    snippet_original_uuid: Uuid,
    parameter_name: String,
    parameter_value: String,
}

impl ProjectManager {
    /// Save the current project session to a file at the specified project file path
    pub fn save_project(
        &mut self,
        external_snippet_manager: &mut ExternalSnippetManager,
        path: PathBuf,
    ) -> Result<(), String> {
        // get components
        let snippet_manager = &mut self.snippet_manager;

        // create plan
        let mut plan = Plan::default();
        // build plan

        // build snippets plan
        // build snippet actions
        for snippet in snippet_manager.get_snippets_as_ref() {
            // find it in external snippet manager
            let external_snippet = match external_snippet_manager
                .find_external_snippet(snippet.get_external_snippet_id())
            {
                None => {
                    return Err(format!("Could not find snippet in external snippet manager in project build actions step"));
                }
                Some(external_snippet) => external_snippet,
            };

            // get python path
            let package_path = external_snippet.get_package_path();

            // get positions
            let position = snippet.get_position();
            let x_position = position.0;
            let y_position = position.1;

            // create snippet action
            let snippet_action = BuildSnippetAction {
                package_path: package_path,
                original_uuid: snippet.get_uuid(),
                x_position: x_position,
                y_position: y_position,
            };

            // add the created snippet action to the list of build snippet actions in the plan
            plan.actions.build_snippet_actions.push(snippet_action);
        }

        // for each snippet
        for snippet in snippet_manager.get_snippets_as_ref() {
            //  for each snippet connector
            for snippet_connector_uuid in snippet.get_pipeline_connector_uuids() {
                //   for each pipeline find_pipeline_uuids_from_pipeline_connector
                for pipeline_uuid in snippet_manager
                    .find_pipeline_uuids_from_pipeline_connector(&snippet_connector_uuid)
                {
                    // get pipeline
                    let pipeline = match snippet_manager.find_pipeline(&pipeline_uuid) {
                        None => return Err(format!("Critical logic error in get pipelines from to connector: Pipeline not found")),
                        Some(pipeline) => pipeline,
                    };

                    // get from and to pipeline connector uuid
                    let from_pipeline_connector_uuid = pipeline.get_from_pipeline_connector_uuid();
                    let to_pipeline_connector_uuid = pipeline.get_to_pipeline_connector_uuid();

                    // only proceed if the snippet connecotr id is the from pipeline connector uuid to not duplicate efforts
                    if snippet_connector_uuid != from_pipeline_connector_uuid {
                        continue;
                    }

                    //    find_snippet_uuid_from_pipeline_connector
                    let connecting_snippet_uuid = match snippet_manager
                        .find_snippet_uuid_from_pipeline_connector(&to_pipeline_connector_uuid)
                    {
                        Some(uuid) => uuid,
                        None => {
                            return Err(format!("Critical logic error: Cannot find snippet Uuid from pipeline connector"))
                        }
                    };

                    // get connecting snippet
                    let connecting_snippet =
                        match snippet_manager.find_snippet(&connecting_snippet_uuid) {
                            Some(snippet) => snippet,
                            None => return Err(format!("Critical logic error: Snippet not found")),
                        };

                    // get pipeline connector names
                    let from_pipeline_connector_name =
                        match snippet.find_pipeline_connector(from_pipeline_connector_uuid) {
                            Some(pipeline_connector) => pipeline_connector.get_name(),
                            None => {
                                return Err(String::from(
                                    "Critical logic error in getting from pipeline from pipeline connector: Pipeline connector not found",
                                ))
                            }
                        };

                    // get pipeline connector names
                    let to_pipeline_connector_name =
                        match connecting_snippet.find_pipeline_connector(to_pipeline_connector_uuid) {
                            Some(pipeline_connector) => pipeline_connector.get_name(),
                            None => {
                                return Err(String::from(
                                    "Critical logic error in getting to pipeline connector : Pipeline connectornot found",
                                ))
                            }
                        };

                    // get package paths
                    let from_snippet_python_path = match external_snippet_manager
                        .find_external_snippet(snippet.get_external_snippet_id())
                    {
                        None => {
                            return Err(format!("Could not find snippet in external snippet manager in project build actions step"));
                        }
                        Some(external_snippet) => external_snippet.get_package_path(),
                    };

                    let to_snippet_python_path = match external_snippet_manager
                        .find_external_snippet(connecting_snippet.get_external_snippet_id())
                    {
                        None => {
                            return Err(format!("Could not find snippet in external snippet manager in project build actions step"));
                        }
                        Some(external_snippet) => external_snippet.get_package_path(),
                    };

                    // create and add entry
                    plan.actions
                        .build_snippet_pipeline_actions
                        .push(BuildSnippetPipelineAction {
                            from_snippet_package_path: from_snippet_python_path,
                            from_snippet_original_uuid: snippet.get_uuid(),
                            from_snippet_connector_name: from_pipeline_connector_name,
                            to_snippet_package_path: to_snippet_python_path,
                            to_snippet_original_uuid: connecting_snippet.get_uuid(),
                            to_snippet_connector_name: to_pipeline_connector_name,
                        });
                }
            }
        }

        // add parameter values
        // for each snippet
        for snippet in snippet_manager.get_snippets_as_ref() {
            // for each parameter
            for parameter in snippet.get_parameters_as_copy() {
                let parameter_name = parameter.get_name();

                // get inner value as string
                let parameter_value = parameter.get_storage().to_string();

                // get package path
                let snippet_package_path = match external_snippet_manager
                    .find_external_snippet(snippet.get_external_snippet_id())
                {
                    None => {
                        return Err(format!("Could not find snippet in external snippet manager in project build actions step"));
                    }
                    Some(external_snippet) => external_snippet.get_package_path(),
                };

                // add entry to plan.build_snippet_parameter_actions
                plan.actions
                    .build_snippet_parameter_actions
                    .push(BuildSnippetParameterAction {
                        snippet_package_path: snippet_package_path,
                        snippet_original_uuid: snippet.get_uuid(),
                        parameter_name: parameter_name,
                        parameter_value: parameter_value,
                    });
            }
        }

        // serialize plan
        let serialized_plan = match bincode::serialize(&plan) {
            Ok(some) => some,
            Err(e) => {
                return Err(format!("Unable to serialize plan: {}", e));
            }
        };

        // create necessary directories for file
        if let Some(parent_dir) = path.parent() {
            match std::fs::create_dir_all(parent_dir) {
                Ok(()) => (),
                Err(e) => {
                    return Err(format!(
                        "Unable to create necessary directories for project file {}: {}",
                        path.to_string_lossy(),
                        e
                    ));
                }
            }
        } else {
            return Err("Invalid project file path".to_string());
        }

        // create file, truncate if exists
        let mut file = match std::fs::File::create(path.to_owned()) {
            Ok(some) => some,
            Err(e) => {
                return Err(format!(
                    "Unable to create project file at {}: {}",
                    path.to_string_lossy(),
                    e
                ));
            }
        };

        // export plan
        match file.write_all(&serialized_plan) {
            Ok(()) => (),
            Err(e) => {
                return Err(format!(
                    "Unable to write serialized plan to file {}: {}",
                    path.to_string_lossy(),
                    e
                ));
            }
        };

        return Ok(());
    }

    /// Read the project from the project file path
    ///
    /// returns the project build information
    pub fn open_project(&self, path: PathBuf) -> Result<Plan, String> {
        // create file, truncate if exists
        let mut file = match std::fs::File::open(path.to_owned()) {
            Ok(some) => some,
            Err(e) => {
                return Err(format!(
                    "Unable to read project file at {}: {}",
                    path.to_string_lossy(),
                    e
                ));
            }
        };

        let mut unserialized_plan = Vec::<u8>::new();

        // export plan
        match file.read_to_end(&mut unserialized_plan) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!(
                    "Unable to read unserialized plan from the file {}: {}",
                    path.to_string_lossy(),
                    e
                ));
            }
        };

        // deserialize plan
        let plan: Plan = match bincode::deserialize(&unserialized_plan) {
            Ok(some) => some,
            Err(e) => {
                return Err(format!("Unable to serialize plan: {}", e));
            }
        };

        return Ok(plan);
    }

    pub fn get_default_plan(&self) -> Plan {
        return Plan::default();
    }
}

/// Get the directory path of project given it's name
pub fn get_project_directory_location_from_name(project_name: String) -> PathBuf {
    // get working directory
    let mut project_path = get_projects_directory();

    // get project name path
    let project_name_path: std::vec::Vec<String> =
        project_name.split('.').map(|s| s.to_string()).collect();

    // build path
    for project_name_part in project_name_path {
        project_path = project_path.join(project_name_part);
    }

    // append extension
    project_path.set_extension("project");

    return project_path;
}
