<script>
    import { invoke } from "@tauri-apps/api";
    import { createEvent } from "konva/lib/PointerEvents";
    import { createEventDispatcher } from "svelte";
    
    //for dispatching events
    const dispatch = createEventDispatcher();

    export let id = -1;
    export let name = "";
    export let level = 0;
    export let file_type = "";
    export let is_directory = true;

    let expanded = false;

    function onDragStart(e) {
        e.dataTransfer.setData('id', e.target.getAttribute('id'));
        e.dataTransfer.setData('snippet_id', id);
        e.dataTransfer.setData('type', file_type);
        e.dataTransfer.setData('name', name);
        //e.dataTransfer.setData('pipeline_connectors', pipeline_connectors);

    }

    function onDragEnd(e) {

    }

    function handleClick(e) {
        if (expanded) {
            expanded = false;

            dispatch('contract', {
                id: id
            });
        }
        else {
            //update state to reflect expansion
            expanded = true;

            dispatch('expand', {
                id: id
            });
        }
    }

    /*
    dispatch('expand', {
        id: id
    })
    dispatch('contract', {
        id: id
    })*/
</script>

<!--visual file component with draggable properties-->
<div 
    id={name}
    class="body" 
    style="--indent: {level * 17}px"
    draggable=true
    on:dragstart={onDragStart}
    on:dragend={onDragEnd}
>
    <!--if it is not a directory-->
    {#if is_directory}
        <div class="expandable_arrow" on:click={handleClick}>
            <!--if the directory is expanded-->
            {#if expanded}
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-chevron-down" viewBox="0 0 16 16">
                    <path fill-rule="evenodd" d="M1.646 4.646a.5.5 0 0 1 .708 0L8 10.293l5.646-5.647a.5.5 0 0 1 .708.708l-6 6a.5.5 0 0 1-.708 0l-6-6a.5.5 0 0 1 0-.708z"/>
                </svg>
            {:else}
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-chevron-right" viewBox="0 0 16 16">
                    <path fill-rule="evenodd" d="M4.646 1.646a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L10.293 8 4.646 2.354a.5.5 0 0 1 0-.708z"/>
                </svg>
            {/if}
        </div>
    {:else}
        <svg fill="#000000" width="16px" height="16px" viewBox="0 0 24 24" class="bi bi-circuit-svgrepo" xmlns="http://www.w3.org/2000/svg">
            <path d="M10,13a1,1,0,1,0,1,1A1,1,0,0,0,10,13Zm0-4a1,1,0,1,0,1,1A1,1,0,0,0,10,9Zm4,0a1,1,0,1,0,1,1A1,1,0,0,0,14,9Zm7,4a1,1,0,0,0,0-2H19V9h2a1,1,0,0,0,0-2H18.82A3,3,0,0,0,17,5.18V3a1,1,0,0,0-2,0V5H13V3a1,1,0,0,0-2,0V5H9V3A1,1,0,0,0,7,3V5.18A3,3,0,0,0,5.18,7H3A1,1,0,0,0,3,9H5v2H3a1,1,0,0,0,0,2H5v2H3a1,1,0,0,0,0,2H5.18A3,3,0,0,0,7,18.82V21a1,1,0,0,0,2,0V19h2v2a1,1,0,0,0,2,0V19h2v2a1,1,0,0,0,2,0V18.82A3,3,0,0,0,18.82,17H21a1,1,0,0,0,0-2H19V13Zm-4,3a1,1,0,0,1-1,1H8a1,1,0,0,1-1-1V8A1,1,0,0,1,8,7h8a1,1,0,0,1,1,1Zm-3-3a1,1,0,1,0,1,1A1,1,0,0,0,14,13Z"/>
        </svg>
    {/if}
    <div class="name tauri-regular">
        {name}
    </div>
</div>

<style>
    .body {
        padding-left: var(--indent);
        padding-top: 3px;
    }

    .bi {
        float: left;
    }

    .name {
        padding-left: 4px;
        user-select: none;
        padding-bottom: 4px;
    }

    .tauri-regular {
        font-family: "Tauri", sans-serif;
        font-weight: 400;
        font-style: normal;
    }

    .expandable_arrow {
        float: left;
    }

</style>