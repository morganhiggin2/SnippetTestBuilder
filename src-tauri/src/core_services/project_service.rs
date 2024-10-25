use std::{
    io::{Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{
    core_components::snippet_manager::SnippetManager,
    state_management::{
        external_snippet_manager::{self, ExternalSnippetManager, PackagePath},
        visual_snippet_component_manager::VisualSnippetComponentManager,
        window_manager::WindowSession,
    },
};

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
struct Plan {
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
    python_path: PackagePath,
}

#[derive(Serialize, Deserialize)]
struct BuildSnippetPipelineAction {
    // (from snippet path, from snippet connector name), (to snippet path, to snippet connector name)
    from_snippet_package_path: PackagePath,
    from_snippet_connector_name: String,
    to_snippet_package_path: PackagePath,
    to_snippet_connector_name: String,
}

#[derive(Serialize, Deserialize)]
struct BuildSnippetParameterAction {
    snippet_package_path: PackagePath,
    parameter_value: String,
}

/// Save the current project session to a file at the specified project file path
pub fn save_project(
    window_session: &mut WindowSession,
    external_snippet_manager: &mut ExternalSnippetManager,
    path: PathBuf,
) -> Result<(), String> {
    // get components
    let snippet_manager = &mut window_session.project_manager.snippet_manager;

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
        let python_path = external_snippet.get_package_path();

        // create snippet action
        let snippet_action = BuildSnippetAction {
            python_path: python_path,
        };

        // add the created snippet action to the list of build snippet actions in the plan
        plan.actions.build_snippet_actions.push(snippet_action);

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
                        None => return Err(format!("Critical logic error: Pipeline not found")),
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
                                    "Critical logic error: Pipeline connector not found",
                                ))
                            }
                        };

                    // get pipeline connector names
                    let to_pipeline_connector_name =
                        match snippet.find_pipeline_connector(to_pipeline_connector_uuid) {
                            Some(pipeline_connector) => pipeline_connector.get_name(),
                            None => {
                                return Err(String::from(
                                    "Critical logic error: Pipeline connector not found",
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
                            from_snippet_connector_name: from_pipeline_connector_name,
                            to_snippet_package_path: to_snippet_python_path,
                            to_snippet_connector_name: to_pipeline_connector_name,
                        });
                }
            }

            // add parameter values
            // for each snippet
            for snippet in snippet_manager.get_snippets_as_ref() {
                // for each parameter
                for parameter in snippet.get_parameters_as_copy() {
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
                    plan.actions.build_snippet_parameter_actions.push(
                        BuildSnippetParameterAction {
                            snippet_package_path: snippet_package_path,
                            parameter_value: parameter_value,
                        },
                    );
                }
            }
        }
    }

    // serialize plan
    let serialized_plan = match bincode::serialize(&plan) {
        Ok(some) => some,
        Err(e) => {
            return Err(format!("Unable to serialize plan: {}", e));
        }
    };

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
fn read_project(path: PathBuf) -> Result<Plan, String> {
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
                "Unable to write serialized plan to file {}: {}",
                path.to_string_lossy(),
                e
            ));
        }
    };

    // deserialize plan
    let plan = match bincode::deserialize(&unserialized_plan) {
        Ok(some) => some,
        Err(e) => {
            return Err(format!("Unable to serialize plan: {}", e));
        }
    };

    return plan;
}
