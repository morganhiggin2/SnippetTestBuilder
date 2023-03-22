<script>
    import {invoke} from '@tauri-apps/api';
    import { onMount } from 'svelte';
    import Konva from 'konva';

    let stage;

    onMount(() => {
        //create stage
        stage = new Konva.Stage({
            container: 'stage',
            width: window.innerWidth,
            height: window.innerHeight,
            draggable: true
        });

        //create bottom layer
        let layer = new Konva.Layer();

        //add objects to layer

        //add layer to stage
        stage.add(layer);
        /*ctx = canvas.getContext("2d");

        ctx.fillStyle = "green";
        ctx.fillRect(10, 10, 150, 150);*/
    });

    //workarea state
    let snippets = [{id: 0, name: "testing snippet"}];

    function handleDrop(e) {
        e.preventDefault();
        //var element_text = e.dataTransfer.getData("text");
        invoke('logln', {text: 'drag drop confirmed of any type'});
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
    <!--<canvas id="canvas" bind:this={canvas}>

    </canvas>-->
    <div class="stage">

    </div>
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