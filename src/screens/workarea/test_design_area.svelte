<script>
    import { onMount } from "svelte";
    import TestCreationArea from "./test_creation_area.svelte";
    import { invoke } from "@tauri-apps/api";
    import ReportArea from "./report_area.svelte";

    // logging
    let trigger_logging_;

    export const trigger_logging = (stream_i) => {
        trigger_logging_(stream_i);
    };

    // project builder
    let create_snippet_;
    let create_pipeline_;

    export const create_snippet = (id, x, y) => {
        create_snippet_(id, x, y);
    };

    export const create_pipeline = () => {
        create_pipeline_();
    };

    // parameters
    let set_parameter_text_;

    export const set_parameter_text = (id, text) => {
        set_parameter_text_(id, text);
    };

    let window_height = 0;
    export let window_session_id;
    export let sidebar_width;

    let src = "";

    // parameters methods
    let add_parameters;
    let delete_parameters;
</script>

<div class="body">
    <div
        class="container"
        style="grid-template-rows: {window_height - 250}px 250px;"
    >
        <div class="test-creation-area">
            <TestCreationArea
                {window_session_id}
                {add_parameters}
                {delete_parameters}
                bind:create_snippet={create_snippet_}
                bind:create_pipeline={create_pipeline_}
            />
        </div>
        <div class="logging-view">
            <ReportArea
                {window_session_id}
                bind:trigger_logging={trigger_logging_}
                bind:add_parameters
                bind:delete_parameters
                bind:set_parameter_text={set_parameter_text_}
                {sidebar_width}
            />
        </div>
    </div>
</div>

<svelte:window bind:innerHeight={window_height} />

<style>
    .body {
        height: 100%;
    }

    .container {
        display: grid;
    }

    .test-creation-area {
        grid-row: 1 / span 1;
    }

    .logging-view {
        grid-row: 2 / span 1;
    }

    .logging-view.border {
        grid-column: 3 / span 1;
        background-color: lightgrey;
        cursor: col-resize;
    }
</style>
