<script>
    import { onMount } from "svelte";
    import PropertiesArea from "./properties_area.svelte";
    import LoggingArea from "./logging_area.svelte";
    import TestCreationArea from "./test_creation_area.svelte";
    import { invoke } from "@tauri-apps/api";

    // logging
    let trigger_logging_;

    export const trigger_logging = (stream_i) => {
        trigger_logging_(stream_i);
    }

    let window_height = 0;
    export let window_session_id;

    let src = "";

</script>

<div class="body">
    <div class="container" style="grid-template-rows: {window_height - 250}px 250px;">
        <div class="test-creation-area">
            <TestCreationArea {window_session_id}/>
        </div>
        <div class="properties-view">
            <LoggingArea {window_session_id} bind:trigger_logging={trigger_logging_}/>
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

    .properties-view {
        grid-row: 2 / span 1;
    }
</style>