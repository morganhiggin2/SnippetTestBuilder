<script>
    import { onMount } from "svelte";
    import PropertiesArea from "./properties_area.svelte";
    import TestCreationArea from "./test_creation_area.svelte";
    import { invoke } from "@tauri-apps/api";

    let window_height = 0;
    let window_session_id = 0;

    let src = "";

    onMount(() => {
        //create new window sesison
        //set id on completion
        invoke('new_window_session')
            .then((result) => {
            window_session_id = result;
        });
    });

</script>

<div class="body">
    <div class="container" style="grid-template-rows: {window_height - 250}px 250px;">
        <div class="test-creation-area">
            <TestCreationArea {window_session_id}/>
        </div>
        <div class="properties-view">
            <PropertiesArea {window_session_id}/>
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