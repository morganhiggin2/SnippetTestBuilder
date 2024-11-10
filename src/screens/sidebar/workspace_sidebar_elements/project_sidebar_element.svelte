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
    export let project_display_id = "";

    function onDragStart(e) {
        var drag_data = {
            _id: id,
            type: file_type,
            display_id: project_display_id,
        };

        e.dataTransfer.setData("text/plain", JSON.stringify(drag_data));
    }

    function onDragEnd(e) {}

    function onDoubleClick() {
        // TODO delete project
        // refresh workspace
        // do this in screen above
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
    draggable="true"
    on:dragstart={onDragStart}
    on:dragend={onDragEnd}
>
    <!--if it is not a parent-->
    <svg
        fill="#000000"
        width="16px"
        height="16px"
        viewBox="0 0 24 24"
        class="bi"
        xmlns="http://www.w3.org/2000/svg"
        ><path
            d="M3,18H7.382L6.105,20.553a1,1,0,0,0,1.79.894L9.618,18H11v2a1,1,0,0,0,2,0V18h1.382l1.723,3.447a1,1,0,1,0,1.79-.894L16.618,18H21a1,1,0,0,0,1-1V5a1,1,0,0,0-1-1H13V3a1,1,0,0,0-2,0V4H3A1,1,0,0,0,2,5V17A1,1,0,0,0,3,18ZM4,6H20V16H4Z"
        /></svg
    >
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
</style>
