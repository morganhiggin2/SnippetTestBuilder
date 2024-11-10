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
    let draw_pipeline_;

    export const create_snippet = (id, x, y) => {
        return create_snippet_(id, x, y);
    };

    export const draw_pipeline = (from_uuid, to_uuid) => {
        draw_pipeline_(from_uuid, to_uuid);
    };

    let window_height = 0;
    export let window_session_id;
    export let sidebar_width;

    let src = "";

    // parameters methods
    let insert_parameters;
    let delete_parameters;
    export let update_parameter_text;

    // project loading
    export let open_project;
    export let clear_visuals;
    export let clear_report_area;

    // properties state
    export let project_properties_state;
</script>

<div class="body">
    <div
        class="container"
        style="grid-template-rows: {window_height - 250}px 250px;"
    >
        <div class="test-creation-area">
            <TestCreationArea
                {window_session_id}
                {insert_parameters}
                {delete_parameters}
                {open_project}
                {project_properties_state}
                bind:create_snippet={create_snippet_}
                bind:draw_pipeline={draw_pipeline_}
                bind:clear_visuals
            />
        </div>
        <div class="logging-view">
            <ReportArea
                {window_session_id}
                bind:trigger_logging={trigger_logging_}
                bind:insert_parameters
                bind:delete_parameters
                bind:update_parameter_text
                bind:project_properties_state
                bind:clear_report_area
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
