<script>
    import { invoke } from "@tauri-apps/api";
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
    var parameters_state = {
        parameters: [] 
    }

    // parameters methods
    export const add_parameters = (snippet_id, parameters) => {
        let param_state_parameters = parameters_state.parameters;

        for (const parameter of parameters) {
            // [snippet id, paramter information, paramter text content as a string]
            param_state_parameters.push([snippet_id, parameter, ""]);
        }

        parameters_state.param_state_parameters = param_state_parameters;
    }

    export const delete_parameters = (snippet_id) => {
        var param_state_parameters = [];

        for (const parameter of parameters_state.parameters) {
            if (parameter[0] != snippet_id) {
                param_state_parameters.push(parameter);
            }
        }

        parameters_state.parameters = param_state_parameters;
    }

</script>

<div class="body">
    <div class="tab-bar">
        <ScreenTabBar screens={screens} active_screen={active_screen} change_screen={change_screen}/>
    </div>
    {#if active_screen == "logging"}
        <LoggingArea {window_session_id} {logging_state} bind:trigger_logging={trigger_logging_}/>
    {:else if active_screen == "parameters"}
        <ParametersArea {window_session_id} bind:parameters_state={parameters_state}/>
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