<script>
    import { invoke } from "@tauri-apps/api";

    export let window_session_id;

    // timeed event so that when the user is done typing, it updates the paramter
    // one event per paramter
    let property_typing_timer = null;
    // in miliseconds
    let typing_interval = 2000;

    // properties
    // workspace name
    export let properties_state;

    // on key up from typing
    function on_key_up_typing() {
        // clear the timeout for the existing paramter based on paramter id
        clearTimeout(property_typing_timer);

        // start timeout for that paramter
        property_typing_timer = setTimeout(() => {
            update_properties();
        }, typing_interval);
    }

    function update_properties() {
        // update paramter text
        /*invoke("update_workspace_properties", {
            windowSessionUuid: window_session_id,
            workspaceName: properties_state.workspace_name,
        })
            .then(() => {})
            .catch((e) => {
                invoke("logln", { text: JSON.stringify(e) });
                });*/
    }
</script>

<div class="body">
    <div class="property tauri-regular">
        <div class="property name">project name</div>
        <div
            class="property value"
            on:keyup={() => {
                on_key_up_typing();
            }}
        >
            <textarea
                class="input-element"
                rows="1"
                bind:value={properties_state.workspace_name}
            />
        </div>
    </div>
</div>

<style>
    .body {
        height: 100%;
        width: 100%;
        background-color: white;
        overflow-y: auto;
        border-top: 1px solid lightgrey;
        padding-left: 4px;
        padding-top: 2px;
    }

    .property {
        display: flex;
        justify-content: flex-start;
        align-items: center;
        margin: 1px;
    }

    .input-element {
        border-radius: 4px;
        border-width: 1px;
        margin: 1px;
    }

    .input-element::selection {
        color: #3776ab;
    }

    .input-element::-moz-selection {
        color: #3776ab;
    }

    .input-element:focus {
        outline: none;
        border-color: #3776ab;
        border-radius: 4px;
        border-width: 2px;
        margin: 0px;
    }

    .property.name {
        padding-right: 0px;
        margin-right: 0px;
        font-size: 13px;
    }

    .property.value {
        padding-left: 4px;
        margin-left: 0px;
    }
</style>
