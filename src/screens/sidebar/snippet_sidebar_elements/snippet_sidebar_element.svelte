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

    function onDragStart(e) {
        e.dataTransfer.setData('_id', id);
        e.dataTransfer.setData('type', file_type);
    }

    function onDragEnd(e) {

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
    style="--indent: {(level - 1) * 17}px"
    draggable=true
    on:dragstart={onDragStart}
    on:dragend={onDragEnd}
>
    <!--if it is not a directory-->
    <svg fill="#000000" width="16px" height="16px" viewBox="0 0 24 24" class="bi bi-circuit-svgrepo" xmlns="http://www.w3.org/2000/svg">
        <path d="M10,13a1,1,0,1,0,1,1A1,1,0,0,0,10,13Zm0-4a1,1,0,1,0,1,1A1,1,0,0,0,10,9Zm4,0a1,1,0,1,0,1,1A1,1,0,0,0,14,9Zm7,4a1,1,0,0,0,0-2H19V9h2a1,1,0,0,0,0-2H18.82A3,3,0,0,0,17,5.18V3a1,1,0,0,0-2,0V5H13V3a1,1,0,0,0-2,0V5H9V3A1,1,0,0,0,7,3V5.18A3,3,0,0,0,5.18,7H3A1,1,0,0,0,3,9H5v2H3a1,1,0,0,0,0,2H5v2H3a1,1,0,0,0,0,2H5.18A3,3,0,0,0,7,18.82V21a1,1,0,0,0,2,0V19h2v2a1,1,0,0,0,2,0V19h2v2a1,1,0,0,0,2,0V18.82A3,3,0,0,0,18.82,17H21a1,1,0,0,0,0-2H19V13Zm-4,3a1,1,0,0,1-1,1H8a1,1,0,0,1-1-1V8A1,1,0,0,1,8,7h8a1,1,0,0,1,1,1Zm-3-3a1,1,0,1,0,1,1A1,1,0,0,0,14,13Z"/>
    </svg>
    <div class="name">
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
    }
</style>