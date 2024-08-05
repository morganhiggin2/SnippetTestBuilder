<script>
    import { invoke } from "@tauri-apps/api";

    export let parameters_state;

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
        let parameter_text = parameters_state.parameters.filter((param) => param[0] == id)[0][2];

        // update paramter text
        invoke('update_snippet_parameter_value', {front_uuid: id, value: parameter_text}).then(() => {});
    }
</script>

<div class="body">
    {#each parameters_state.parameters as parameter} 
        {#if parameter[1].p_type == "SingleLineText"}
            <div class="parameter">
                <div class="parameter name">
                    {parameter[1].name + ":"}
                </div>
                <div class="parameter value">
                    <textarea rows="1" bind:value={parameter[2]}/>
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
        border-top: 2px solid lightgrey;
        overflow-y: auto;
    }

    .parameter {
        display: flex;
        justify-content: flex-start;
        align-items: center;
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