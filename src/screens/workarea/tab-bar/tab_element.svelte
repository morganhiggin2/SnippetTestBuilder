<script>
    import { identity } from "svelte/internal";

    export let id;
    export let text;
    export let active;
    export let change_active_tab;

    function on_click() {
        change_active_tab(id);
    }

    /*
    
    
    .body {
        background-color: whitesmoke;
        display: inline-block;
    }

    
    .text {

        text-decoration: none;
    }
<div class="body">
    <div class="text">
        {text}
    </div>
</div>
    */
</script>
    {#if active}
        <div class="body noselect" id="active" on:click={on_click} on:keydown={() => {}}>
            <div class="text tauri-regular">
                {text}
            </div>
        </div>
    {:else}
        <div class="body noselect" id="inactive" on:click={on_click} on:keydown={() => {}}>
            <div class="text tauri-regular">
                {text}
            </div>
        </div>
    {/if}

<style>
    .body {
        display: inline-block;
        padding: 4px;
        padding-left: 6px;
        padding-right: 8px;
        border-right: 1px solid lightgrey;
        border-bottom: 2px solid whitesmoke;
        margin-bottom: -1px;
        cursor: default;
    }

    .body:hover {
        border-bottom: 2px solid gray;
    }    

    .text {
        padding-bottom: 2px;
        font-size: 13px;
    }

    #active {
        background-color: white;
    }

</style>