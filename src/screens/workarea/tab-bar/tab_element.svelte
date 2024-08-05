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
        <div class="body" id="active" on:click={on_click} on:keydown={() => {}}>
            <div class="text">
                {text}
            </div>
        </div>
    {:else}
        <div class="body" id="inactive" on:click={on_click} on:keydown={() => {}}>
            <div class="text">
                {text}
            </div>
        </div>
    {/if}

<style>
    .body {
        display: inline-block;
        padding: 4px;
        padding-right: 25px;
        border-bottom: 2px solid whitesmoke;
        border-left: 1px solid lightgrey;
        border-right: 1px solid lightgrey;
        margin-bottom: -2px;
        cursor: default;
    }

    .body:hover {
        border-bottom: 2px solid blue;
    }    

    .text {
        padding-bottom: 2px;
    }

    #active {
        background-color: darkgray;
    }
</style>