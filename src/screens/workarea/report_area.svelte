<script>
    import { invoke } from "@tauri-apps/api";
    import LoggingArea from "./report_area/logging_area.svelte";
    import ParametersArea from "./report_area/parameters_area.svelte";
    import PropertiesArea from "./report_area/properties_area.svelte";
    import ScreenTabBar from "./report_area/screen_tab_bar.svelte";

    export var window_session_id;
    export let sidebar_width;

    // logging
    let trigger_logging_;

    export const trigger_logging = (stream_i) => {
        trigger_logging_(stream_i);
    };

    // multi screen
    var screens = ["logging", "parameters", "properties"];
    var active_screen = "logging";

    function change_screen(screen) {
        active_screen = screen;
    }

    //state for each screen
    var logging_state = {
        log_text: "",
    };

    var parameters_state = {
        parameters: new Map(),
    };

    export var project_properties_state = {
        project_name: "",
    };

    // parameters methods
    export const insert_parameters = (snippet_id, parameters) => {
        for (let i = 0; i < parameters.length; i++) {
            const put_parameter = parameters[i];

            let parameter_key = put_parameter.id;

            parameters_state.parameters.set(parameter_key, {
                parameter_information: put_parameter,
                snippet_id: snippet_id,
                value: "",
            });
        }
    };

    export const delete_parameters = (snippet_id) => {
        let for_delete_params = [];

        for (const [
            parameter_key,
            parameter_value,
        ] of parameters_state.parameters) {
            if (parameter_value.snippet_id === snippet_id) {
                for_delete_params.push(parameter_key);
            }
        }

        for (const parameter_key of for_delete_params) {
            parameters_state.parameters.delete(parameter_key);
        }
    };

    export const update_parameter_text = (parameter_id, text) => {
        let parameter_key = parameter_id;

        // only if it contains the key
        if (parameters_state.parameters.has(parameter_key)) {
            // get current value
            let parameter_value =
                parameters_state.parameters.get(parameter_key);
            parameter_value.value = text;

            parameters_state.parameters.set(parameter_key, parameter_value);

            // update paramter text
            invoke("update_snippet_parameter_value", {
                windowSessionUuid: window_session_id,
                frontUuid: parameter_id,
                value: text,
            })
                .then(() => {})
                .catch((e) => {
                    invoke("logln", { text: JSON.stringify(e) });
                });
        }
    };

    export function clear_report_area() {
        //state for each screen
        logging_state = {
            log_text: "",
        };

        parameters_state = {
            parameters: new Map(),
        };
    }
</script>

<div class="body" style="width: calc(100% - {sidebar_width}px);">
    <div class="tab-bar">
        <ScreenTabBar {screens} {active_screen} {change_screen} />
    </div>
    {#if active_screen == "logging"}
        <LoggingArea
            {window_session_id}
            {logging_state}
            bind:trigger_logging={trigger_logging_}
        />
    {:else if active_screen == "parameters"}
        <ParametersArea {window_session_id} bind:parameters_state />}
    {:else if active_screen == "properties"}
        <PropertiesArea {window_session_id} bind:project_properties_state />
    {/if}
</div>

<style>
    .body {
        height: 100%;
        margin-left: -2px;
    }
    .tab-bar {
        height: 26px;
        width: 100%;
    }
</style>
