<script>
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import TabElement from "./tab_element.svelte";

    let tabs = [];

    onMount(() => {
        /*tabs.push({id: 0, text: "main_snippet.snip"});
        tabs.push({id: 1, text: "lower.snip"});
        tabs.push({id: 2, text: "rest_-_users.snip"});
        tabs.push({id: 3, text: "rest_users.snip"});
        tabs.push({id: 4, text: "rest_users.snip"});
        tabs.push({id: 5, text: "rest_users.snip"});
        tabs.push({id: 6, text: "rest_users.snip"});*/

        tabs = tabs;

        //<TabElement {...tab}/> 
    });

    let container;

    function onMouseWheel(event) {
        container.scrollLeft += event.deltaY;
    }
</script>

<div class="body">
    <div class="container" bind:this={container}>
        {#each tabs as tab} 
            <TabElement {...tab}/>
        {/each}
    </div>
</div>

<svelte:window on:wheel={onMouseWheel}/>

<style>
    .body {
        background-color: white;
        border-top: 2px solid lightgrey;
        border-bottom: 2px solid lightgrey;
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