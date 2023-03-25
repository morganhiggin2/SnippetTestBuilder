<script>
    import {invoke} from '@tauri-apps/api';
    import { onMount } from 'svelte';
    import { generateSnippet } from './snippet_module.js';
    import Konva from 'konva';

    //dimensions of the current window
    let window_width = 0;
    let window_height = 0;
    let window_x = 0;
    let window_y = 0;

    let container;

    //for drawing
    let stage;
    let layer;
    let selfObj;

    //for snippets
    let snippetComponents = [];

    onMount(() => {

        //create stage
        stage = new Konva.Stage({
            container: container,
            width: window_width,
            height: window_height,
            draggable: true
        });

        //create snippet layer
        layer = new Konva.Layer({});

        //add layer to stage
        stage.add(layer);

        //draw stage
        stage.draw();
    });

    function handleDrop(e) {
        e.preventDefault();
        //var element_text = e.dataTransfer.getData("text");
        invoke('logln', {text: 'drag drop confirmed of any type'});

        //get the bounding rectagle for canvas
        let boundingRect = selfObj.getBoundingClientRect();

        //set window x and window y
        window_x = boundingRect.left;
        window_y = boundingRect.top;

        //type
        let type = e.dataTransfer.getData('type');

        if (type == 'snippet') {
        
            invoke('logln', {text: e.clientY.toString()});

            //id
            let id = e.dataTransfer.getData('id');

            //create drawable snippet
            let snippetDrawable = generateSnippet(e.clientX - window_x, e.clientY - window_y);

            //create snippet
            snippetComponents.push({id: 1, name: "testing snippet", drawable: snippetDrawable});

            //draw snippet
            drawSnippet(snippetDrawable);
        }
    }

    function drawSnippet(snippetDrawable) {
        layer.add(snippetDrawable);
        stage.draw();
    }
</script>

<svelte:window bind:innerWidth={window_width} bind:innerHeight={window_height}/>

<div class="body" on:drop={handleDrop} on:dragover|preventDefault={() => {return false;}} bind:this={selfObj}>
    <!--{#each snippets as snippet}
        <p>
            this is a snippet
        </p>
    {/each}-->
    <!--<canvas id="canvas" bind:this={canvas}>

    </canvas>-->
    <div class="stage" bind:this={container}>

    </div>
</div>

<style>
    .body {
        background-color: white; 
        height: 100%;
    }
</style>