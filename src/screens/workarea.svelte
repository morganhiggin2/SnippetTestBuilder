<script>
    import {invoke} from '@tauri-apps/api';
    import { onMount } from 'svelte';

    onMount(() => {
        ctx = canvas.getContext("2d");

        ctx.fillStyle = "green";
        ctx.fillRect(10, 10, 150, 150);
    });

    //canvas
    let canvas;
    let ctx;

    //workarea state
    let snippets = [{id: 0, name: "testing snippet"}];

    function handleDrop(e) {
        e.preventDefault();
        //var element_text = e.dataTransfer.getData("text");

        //type
        let type = e.dataTransfer.getData('type');

        if (type == 'snippet') {
            //id
            let id = e.dataTransfer.getData('id');

            //create snippet
            snippets.push({id: 1, name: "testing snippet"});

            invoke('logln', {text: 'drag drop confirmed of snippet'});

            //have reactivity update 
            snippets = snippets;
        }

    }
</script>

<div class="body" on:drop={handleDrop} on:dragover|preventDefault={() => {return false;}}>
    <!--{#each snippets as snippet}
        <p>
            this is a snippet
        </p>
    {/each}-->
    <canvas id="canvas" bind:this={canvas}>

    </canvas>
</div>

<style>
    .body {
        background-color: white; 
        height: 100%;
        border-top: 2px solid lightgrey;
    }
    .snippet {
        min-width: 100px;
        min-height: 100px;
        background-color: brown;
    }
</style>