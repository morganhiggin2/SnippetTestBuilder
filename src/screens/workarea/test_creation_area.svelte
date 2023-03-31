<script>
    import {invoke} from '@tauri-apps/api';
    import { onMount } from 'svelte';
    import { generatePipeConnector, generateSnippet } from './snippet_module.js';
    import Konva from 'konva';

    //dimensions of the current window
    let window_width = 0;
    let window_height = 0;
    let window_x = 0;
    let window_y = 0;

    let container;

    //for drawing
    let stage;
    let snippetLayer;
    let pipelineLayer;
    let selfObj;

    //for snippets
    let snippetComponents = [];

    //hash map of all visually connected id'd components
    let visualComponents = {};

    onMount(() => {

        //create stage
        stage = new Konva.Stage({
            id: "stage",
            container: container,
            width: window_width,
            height: window_height,
            draggable: true
        });
        
        //on canvas click event
        stage.on('click', handleCanvasClick);

        //create snippet layer
        snippetLayer = new Konva.Layer({});
        pipelineLayer = new Konva.Layer({});

        //add layer to stage
        stage.add(snippetLayer);
        stage.add(pipelineLayer);

        //draw stage
        stage.draw();
    });

    function handleDrop(e) {
        e.preventDefault();
        //var element_text = e.dataTransfer.getData("text");
        //invoke('logln', {text: 'drag drop confirmed of any type'});

        //get the bounding rectagle for canvas
        let boundingRect = selfObj.getBoundingClientRect();

        //set window x and window y
        window_x = boundingRect.left;
        window_y = boundingRect.top;

        //type
        let type = e.dataTransfer.getData('type');

        if (type == 'snippet') {
            //id
            let id = e.dataTransfer.getData('id');

            //create pipeline connectors
            let pipelineConnectors = [
                {id: 0, name: "input", input: true},
                {id: 1, name: "conditioner", input: true},
                {id: 2, name: "output", input: false}
            ];

            //create drawable snippet
            let snippetDrawable = generateSnippet(-1 * snippetComponents.length, visualComponents, e.clientX - window_x, e.clientY - window_y, pipelineConnectors, spawnPipeline, snippetDragStart, snippetDragEnd);

            //create snippet
            snippetComponents.push({id: 1, name: "testing snippet", drawable: snippetDrawable});

            //draw snippet
            drawSnippet(snippetDrawable);
        }
    }

    //draws snippet
    function drawSnippet(snippetDrawable) {
        snippetLayer.add(snippetDrawable);
        //stage.draw();
    }

    //handle pipeline creation event
    //if a pipeline is in the process of being created
    var pipelineInCreationEvent = null;

    function spawnPipeline(pipeline_connector_visual, position_offset) {
        console.log("here");
        //if a pipeline is curerntly not being created
        if (!pipelineInCreationEvent) {
            //check if connector already has pipeline attached (assuming one-to-one policy for now, will be changed in future)
                //return

            //get background rect
            var background_rect = pipeline_connector_visual.getChildren(function(node) {
                return node.getId() === "background_rect";
            })[0];

            var background_rect_position = background_rect.getAbsolutePosition(stage);
            
            //get in creation pipeline id from rust
            var id = "a" + Math.random().toString();

            //start pipeline creation
            pipelineInCreationEvent = {
                visualCompnent: generatePipeConnector(id, visualComponents, background_rect_position.x + position_offset.x, background_rect_position.y + position_offset.y, deletePipeline),
                pipelineConnectorVisual: pipeline_connector_visual,
                start_pos: {
                    x: background_rect_position.x + position_offset.x,
                    y: background_rect_position.y + position_offset.y
                } 
            };

            //draw pipeline
            pipelineLayer.add(pipelineInCreationEvent.visualCompnent);
            stage.draw();
        }
        else {
            //if parent pipeline id is the same, cancel
            var pipelineInCreationVisualComponentParent = pipelineInCreationEvent.pipelineConnectorVisual.getParent();
            var pipelineVisualComponentCurrentParent = pipeline_connector_visual.getParent();

            if (pipelineInCreationVisualComponentParent && pipelineVisualComponentCurrentParent && pipelineInCreationVisualComponentParent.getId() === pipelineVisualComponentCurrentParent.getId()) {
                pipelineInCreationEvent.visualCompnent.destroy();
                pipelineInCreationEvent = null;               
                return;
            } 

            //if before pipeline the same, then cancel 
            if (pipelineInCreationEvent.visualCompnent.getId() === pipeline_connector_visual.getId()) {
                pipelineInCreationEvent.visualCompnent.destroy();
                pipelineInCreationEvent = null;
                return;
            }

            //else, try to connect
                //validate connection
                //change colors of pipelines
                /*pipeline_connector_visual.getChildren(function(node) {
                    return node.getId() === "background_rect";
                })[0].fill('#fcd777');
                pipelineInCreationEvent.visualCompnent.fill('#fcd777');*/
               
                //get current pipeline connector position
                var background_rect = pipeline_connector_visual.getChildren(function(node) {
                    return node.getId() === "background_rect";
                })[0];

                var background_rect_position = background_rect.getAbsolutePosition(stage);

                //correct final position of pipeline
                pipelineInCreationEvent.visualCompnent.points([
                    0, 
                    0, 
                    background_rect_position.x + position_offset.x - pipelineInCreationEvent.start_pos.x, 
                    background_rect_position.y + position_offset.y - pipelineInCreationEvent.start_pos.y
                ]);


                //clear event
                pipelineInCreationEvent = null;

            //else, if the same, cancel pipeline connection
        }
    }

    //handle delete pipeline event from pipeline double click
    function deletePipeline(pipeline_visual) {
        
    }

    let snippetDragEvent = null;

    //for snippet drag event
    function snippetDragStart(id) {
        //create snippet drag start event
        //start position

        //get all pipelines associated with it
    }

    function snippetDragEnd(id) {
        //remove snippet drag event
    }

    function handleMouseMovement(e) {
        if (pipelineInCreationEvent) {
            pipelineInCreationEvent.visualCompnent.points([
                0, 
                0, 
                e.clientX - window_x - pipelineInCreationEvent.start_pos.x, 
                e.clientY - window_y - pipelineInCreationEvent.start_pos.y
            ]);

            stage.draw();
            //redraw pipeline to connect to where mouse is
        }
        else if (snippetDragEvent) {
            //get mouse displacement from start position
            
            //apply to all ends of pipelines
        }

    }

    function handleCanvasClick(e) {
        console.log(e.target.attrs.id);
        if (pipelineInCreationEvent) {
            //if the stage is clicked
            //change to not in hashmap
            if (e.target.attrs.id === "stage") {
                //cancel pipeline creation
                pipelineInCreationEvent.visualCompnent.destroy();
                pipelineInCreationEvent = null;
            }
        }
        //pipeline is in creation and stage is clicked
    }

    //change to when click on, its orange, and then other color changes to orange too when pipe made
    //when snippet is moving, pipes turn into something else, simpler, like partially transparent dotted lines

    //implement grid lock, and grid background maybe?

    //prevent snippet overlap, ihave list of rectangles that are fit for the grid space they are in! then check for overlap
    //only check on mouse release, when placing it, so on dropend
    //in the session manager, have a virtualgridspace manager that checks for colissions

    //migrate to hash map based system
    
    //funcationlize / split up code
</script>

<svelte:window bind:innerWidth={window_width} bind:innerHeight={window_height}/>

<div class="body" on:drop={handleDrop} on:dragover|preventDefault={() => {return false;}} on:mousemove={handleMouseMovement} bind:this={selfObj}>
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