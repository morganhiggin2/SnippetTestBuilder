<script>
    import LoggingArea from "./report_area/logging_area.svelte";
    import ParametersArea from "./report_area/parameters_area.svelte";
    import ScreenTabBar from "./report_area/screen_tab_bar.svelte";

    export var window_session_id;

    // logging
    let trigger_logging_;

    export const trigger_logging = (stream_i) => {
        trigger_logging_(stream_i);
    }

    // multi screen
    var screens = ["logging", "parameters"];
    var active_screen = "logging";

    function change_screen(screen) {
        active_screen = screen;
    }

    //state for each screen
    var logging_state = {
        log_text: ""
    }
    var parameters_state = {}

</script>

<div class="body">
    <div class="tab-bar">
        <ScreenTabBar screens={screens} active_screen={active_screen} change_screen={change_screen}/>
    </div>
    {#if active_screen == "logging"}
        <LoggingArea {window_session_id} {logging_state} bind:trigger_logging={trigger_logging_}/>
    {:else if active_screen == "parameters"}
        <ParametersArea {parameters_state}/>
    {/if}
</div>

<style>
    .body {
        height: 100%;
    }
    .tab-bar {
        height: 26px;
        width: 100%;
    }
</style>