<script>
    import TabBar from "./tab-bar/tab_bar.svelte";
    import TestDesignArea from "./test_design_area.svelte";

    export let window_session_id;    

    let sessions = [];

    // logging
    let trigger_logging_;

    export const trigger_logging = (stream_i) => {
        trigger_logging_(stream_i);
    }
</script>

<div class="body">
    <div class="container">
        <!--
        <div class="tab-bar">
            <TabBar/>
        </div>-->
        <div class="design-area">
            <TestDesignArea window_session_id={window_session_id} bind:trigger_logging={trigger_logging_}/>
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