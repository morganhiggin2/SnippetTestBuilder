<script>
    import { onMount } from "svelte";
    import TestCreationArea from "./test_creation_area.svelte";
    import { invoke } from "@tauri-apps/api";
    import ReportArea from "./report_area.svelte";

    // logging
    let trigger_logging_;

    export const trigger_logging = (stream_i) => {
        trigger_logging_(stream_i);
    }

    let window_height = 0;
    export let window_session_id;
    export let sidebar_width;

    let src = "";

    // parameters methods
    let add_parameters;
    let delete_parameters;



</script>

<div class="body">
    <div class="container" style="grid-template-rows: {window_height - 250}px 250px;">
        <div class="test-creation-area">
            <TestCreationArea {window_session_id} add_parameters={add_parameters} delete_parameters={delete_parameters}/>
        </div>
        <div class="logging-view">
            <ReportArea {window_session_id} bind:trigger_logging={trigger_logging_} bind:add_parameters={add_parameters} bind:delete_parameters={delete_parameters} sidebar_width={sidebar_width}/>
        </div>
    </div>
</div>

<svelte:window bind:innerHeight={window_height}/>

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