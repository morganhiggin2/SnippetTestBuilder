<script>
    import { invoke } from "@tauri-apps/api";
    import TabBar from "./tab-bar/tab_bar.svelte";
    import TestDesignArea from "./test_design_area.svelte";

    export let window_session_id;
    export let sidebar_width;

    let sessions = [];

    // logging
    let trigger_logging_;

    export const trigger_logging = (stream_i) => {
        trigger_logging_(stream_i);
    };

    let create_snippet;
    let create_pipeline;
    let set_parameter_text;

    async function open_project() {
        // get build plan
        let plan = await invoke("open_project", {
            windowSessionId: window_session_id,
        });

        let actions = plan.actions;
        // call actions to create build plan

        // build map of package path to external snippet id
        let package_path_to_visual_id = {};

        // for each snippet
        for (let i = 0; i < actions.buildSnippetActions.length; i++) {
            let snippet_build_action = actions.buildSnippetActions[i];

            //  find external snippet id
            let directory_id = await invoke(
                "get_directory_id_from_package_path",
                {
                    snippetPath: snippet_build_action.packagePath,
                },
            );

            // create snippet, record front snippet id
            let visual_id = await create_snippet(
                directory_id,
                snippet_build_action.positionX,
                snippet_build_action.positionY,
            );

            // insert into mapping
            package_path_to_visual_id[snippet_build_action.packagePath] =
                visual_id;
        }

        // for each pipelines
        for (let i = 0; i < actions.buildSnippetActions.length; i++) {
            let pipeline_build_action = actions.buildSnippetPipelineActions[i];

            //  get from visual front snippet connector
            let from_snippet_connector_id = invoke(
                "get_front_snippet_connector_id_from_snippet_uuid_and_name",
                {
                    windowSessionUuid: window_session_id,
                    frontSnippetId:
                        package_path_to_visual_id[
                            pipeline_build_action.fromSnippetConnectorPath
                        ],
                    snippetConnectorName:
                        pipeline_build_action.fromSnippetConnectorName,
                },
            );

            //  get from visual to snippet connector
            let to_snippet_connector_id = invoke(
                "get_front_snippet_connector_id_from_snippet_uuid_and_name",
                {
                    windowSessionUuid: window_session_id,
                    frontSnippetId:
                        package_path_to_visual_id[
                            pipeline_build_action.toSnippetConnectorPath
                        ],
                    snippetConnectorName:
                        pipeline_build_action.toSnippetConnectorName,
                },
            );

            //  create pipeline
            await create_pipeline(
                from_snippet_connector_id,
                to_snippet_connector_id,
            );
        }

        // for each parameter
        for (let i = 0; i < actions.buildSnippetActions.length; i++) {
            let parameter_build_action = actions.buildSnippetPipelineActions[i];

            let parameter_front_uuid = invoke(
                "get_front_parameter_id_from_snippet_uuid_and_name",
                {
                    windowSessionUuid: window_session_id,
                    frontSnippetId:
                        package_path_to_visual_id[
                            parameter_build_action.fromSnippetConnectorPath
                        ],
                    pipelineName: parameter_build_action.parameterName,
                },
            );

            // update parameter value
            set_parameter_text(
                parameter_front_uuid,
                parameter_build_action.parameterValue,
            );
        }
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
                bind:create_pipeline
                bind:set_parameter_text
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
