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

    function open_project() {
        // get build plan
        invoke("open_project", { windowSessionId: window_session_id }).then(
            (plan) => {
                let actions = plan.actions;
                // call actions to create build plan

                // for each snippet
                for (let i = 0; i < actions.buildSnippetActions.length; i++) {
                    let snippet_build_action = actions.buildSnippetActions[i];

                    //  find external snippet id

                    //  create snippet, record front snippet id
                }

                //  find external snippet id
                //  create snippet, record front snippet id
                // for each pipelines
                //  get from snippet connector
                //  get to snippet connector
                //  create pipelines
            },
        );
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
