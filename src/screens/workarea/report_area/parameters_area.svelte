<script>
    import { invoke } from "@tauri-apps/api";

    export let parameters_state;
    export let window_session_id;

    /*
    {#each parameters_state.parameters as { snippet_id, parameter }, i}
        <div>
            {parameter.text}
        </div>
    {/each}*/

    // timeed event so that when the user is done typing, it updates the paramter
    // one event per paramter
    let parameter_typing_timers = {} 
    // in miliseconds
    let typing_interval = 2000;

    // on key up from typing
    function on_key_up_typing(id) {
        // clear the timeout for the existing paramter based on paramter id
        if (id in parameter_typing_timers) {
            clearTimeout(parameter_typing_timers[id]);
        }

        // start timeout for that paramter
        parameter_typing_timers[id] = setTimeout(() => {update_parameter(id)}, typing_interval);
    }

    function update_parameter(id) {
        // get parameter text
        let parameter_text = parameters_state.parameters.filter((param) => param[1]['id'] == id)[0][2];

        // update paramter text
        invoke('update_snippet_parameter_value', {windowSessionUuid: window_session_id, frontUuid: id, value: parameter_text});
    }
</script>

<div class="body">
    {#each parameters_state.parameters as parameter} 
        {#if parameter[1].p_type == "SingleLineText"}
            <div class="parameter tauri-regular">
                <div class="parameter name">
                    {parameter[1].name}
                </div>
                <div class="parameter value" on:keyup={() => {on_key_up_typing(parameter[1].id)}}>
                    <textarea class="input-element" rows="1" bind:value={parameter[2]}/>
                </div>
            </div>
        {/if}
    {/each}
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

    .parameter {
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

    .parameter.name {
        padding-right: 0px;
        margin-right: 0px;
    }

    .parameter.value {
        padding-left: 4px;
        margin-left: 0px;
    }

</style>