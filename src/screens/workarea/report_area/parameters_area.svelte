<script>
    import { invoke } from "@tauri-apps/api";

    export let parameters_state;

    /*
    {#each parameters_state.parameters as { snippet_id, parameter }, i}
        <div>
            {parameter.text}
        </div>
    {/each}*/
</script>

<div class="body">
    {#each parameters_state.parameters as parameter} 
        {#if parameter[1].p_type == "SingleLineText"}
            <div class="parameter">
                <div class="parameter name">
                    {parameter[1].name + ":"}
                </div>
                <div class="parameter value">
                    <textarea rows="1"/>
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