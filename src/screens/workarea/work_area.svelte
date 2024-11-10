<script>
    import { invoke } from "@tauri-apps/api";
    import TabBar from "./tab-bar/tab_bar.svelte";
    import TestDesignArea from "./test_design_area.svelte";

    export let window_session_id;
    export let sidebar_width;

    let sessions = [];

    // logging
    let trigger_logging_;

    // properties state
    export let project_properties_state;

    // parameters
    let update_parameter_text;

    export const trigger_logging = (stream_i) => {
        trigger_logging_(stream_i);
    };

    let create_snippet;
    let draw_pipeline;

    // for open project
    let clear_visuals;
    let clear_report_area;

    // for delete project
    export let register_listen_to_workspace_refresh;

    async function open_project(window_session_id, project_id) {
        // get build plan
        let plan = {};

        plan = await invoke("open_project", {
            windowSessionUuid: window_session_id,
            projectId: project_id,
        });

        // clear visuals
        clear_visuals();

        // clear report area
        clear_report_area();

        let actions = plan.actions;
        // call actions to create build plan

        // build map of package path to external snippet id
        let package_path_to_visual_id = {};

        // for each snippet
        for (let i = 0; i < actions.build_snippet_actions.length; i++) {
            let snippet_build_action = actions.build_snippet_actions[i];

            //  find external snippet id
            let directory_id = await invoke(
                "get_directory_id_from_package_path",
                {
                    snippetPath: snippet_build_action.package_path.path,
                },
            );

            // create snippet, record front snippet id
            let visual_id = await create_snippet(
                directory_id,
                snippet_build_action.x_position,
                snippet_build_action.y_position,
            );

            // insert into mapping
            package_path_to_visual_id[
                (snippet_build_action.package_path.path,
                snippet_build_action.original_uuid)
            ] = visual_id;
        }

        // for each pipelines
        for (
            let i = 0;
            i < actions.build_snippet_pipeline_actions.length;
            i++
        ) {
            let pipeline_build_action =
                actions.build_snippet_pipeline_actions[i];

            let from_snippet_connector_id = await invoke(
                "get_front_snippet_connector_id_from_snippet_uuid_and_name",
                {
                    windowSessionUuid: window_session_id,
                    frontSnippetId:
                        package_path_to_visual_id[
                            (pipeline_build_action.from_snippet_package_path
                                .path,
                            pipeline_build_action.from_snippet_original_uuid)
                        ],
                    snippetConnectorName:
                        pipeline_build_action.from_snippet_connector_name,
                },
            );

            //  get from visual to snippet connector
            let to_snippet_connector_id = await invoke(
                "get_front_snippet_connector_id_from_snippet_uuid_and_name",
                {
                    windowSessionUuid: window_session_id,
                    frontSnippetId:
                        package_path_to_visual_id[
                            (pipeline_build_action.to_snippet_package_path.path,
                            pipeline_build_action.to_snippet_original_uuid)
                        ],
                    snippetConnectorName:
                        pipeline_build_action.to_snippet_connector_name,
                },
            );

            //  create pipeline
            await draw_pipeline(
                from_snippet_connector_id,
                to_snippet_connector_id,
            );
        }

        // for each parameter
        for (
            let i = 0;
            i < actions.build_snippet_parameter_actions.length;
            i++
        ) {
            let parameter_build_action =
                actions.build_snippet_parameter_actions[i];

            let parameter_front_uuid = await invoke(
                "get_front_parameter_id_from_snippet_uuid_and_name",
                {
                    windowSessionUuid: window_session_id,
                    frontSnippetId:
                        package_path_to_visual_id[
                            (parameter_build_action.snippet_package_path.path,
                            parameter_build_action.snippet_original_uuid)
                        ],
                    parameterName: parameter_build_action.parameter_name,
                },
            );

            // update parameter value
            update_parameter_text(
                parameter_front_uuid,
                parameter_build_action.parameter_value,
            );
        }
    }

    export function delete_project(project_id) {
        invoke("delete_project", {
            windowSessionUuid: window_session_id,
            projectId: project_id,
        }).then(() => {
            register_listen_to_workspace_refresh();

            // spawn refresh workspace
            invoke("spawn_refresh_workspace_event", {
                windowSessionUuid: window_session_id,
            });
            //TODO refresh workspace
        });
    }
</script>

<div class="body">
    <div class="container">
        <!--
        <div class="tab-bar">
            <TabBar/>
        </div>-->
        <div class="design-area">
            <TestDesignArea
                {window_session_id}
                bind:trigger_logging={trigger_logging_}
                bind:create_snippet
                bind:draw_pipeline
                bind:update_parameter_text
                bind:project_properties_state
                bind:clear_visuals
                bind:clear_report_area
                {open_project}
                {sidebar_width}
            />
        </div>
    </div>
</div>

<style>
    .body {
        height: 100%;
    }

    .container {
        display: grid;
        grid-template-rows: auto auto;
    }

    .tab-bar {
        grid-row: 1 / span 1;
    }

    .design-area {
        grid-row: 2 / span 1;
    }
</style>
