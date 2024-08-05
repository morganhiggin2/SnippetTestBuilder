<script>
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import TabElement from "../tab-bar/tab_element.svelte";

    let tabs = [];
    export var screens;
    export var active_screen;
    export var change_screen;

    onMount(() => {
        for (const [i, screen] of screens.entries()) {
            var is_active = active_screen == screen;
            tabs.push({id: i, text: screen, active: is_active});
        }

        tabs = tabs;
    });

    let container;

    function onMouseWheel(event) {
        container.scrollLeft += event.deltaY;
    }

    // for changing active tab
    function change_active_tab(id) {
        active_screen = screens[id]; 

        // redo screens list
        let new_tabs = [];

        for (const [i, screen] of screens.entries()) {
            var is_active = active_screen == screen;
            new_tabs.push({id: i, text: screen, active: is_active});
        }

        tabs = new_tabs;

        change_screen(active_screen);
    }
</script>

<div class="body">
    <div class="container" bind:this={container}>
        {#each tabs as tab} 
            <TabElement {...tab} change_active_tab={change_active_tab}/>
        {/each}
    </div>
</div>

<svelte:window on:wheel={onMouseWheel}/>

<style>
    .body {
        background-color: white;
        border-top: 2px solid lightgrey;
        overflow: hidden;
    }

    .container {
        flex: 1;
        overflow: hidden;
        white-space: nowrap;
        box-sizing: content-box;
        margin-bottom: -2px;
    }

    .container:hover {
        overflow-x: scroll;
    }

    .container::-webkit-scrollbar{
        display: none;
    }
    
</style>