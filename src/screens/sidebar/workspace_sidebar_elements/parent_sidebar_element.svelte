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

    let expanded = false;

    function onDragStart(e) {
        e.dataTransfer.setData('id', id);
        e.dataTransfer.setData('type', file_type);
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
    class="body noselect" 
    style="--indent: {(level - 1) * 17}px"
    draggable=true
    on:dragstart={onDragStart}
    on:dragend={onDragEnd}
>
    <div class="expandable_arrow" on:click={handleClick} on:keypress={() => {}}>
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
        padding-bottom: 2px;
    }

    .expandable_arrow {
        float: left;
    }

</style>