<script>
    import {invoke} from '@tauri-apps/api';
    import { onMount } from 'svelte';
    import { generatePipeConnector, generateSnippet, getChild } from './snippet_module.js';
    import Konva from 'konva';
    import { pipeline } from 'stream';

    export let window_session_id;

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

    onMount(() =>
    {
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

        //get the bounding rectagle for canvas
        let boundingRect = selfObj.getBoundingClientRect();

        //set window x and window y
        window_x = boundingRect.left;
        window_y = boundingRect.top;

        //type
        let type = e.dataTransfer.getData('type');

        if (type == 'Snippet') {
            //get snippet information
            //parsing certain values as everything is passed as string
            let id = JSON.parse(e.dataTransfer.getData('id'));
            let name = e.dataTransfer.getData('name');
            let type = e.dataTransfer.getData('type');
            let internal_id = JSON.parse(e.dataTransfer.getData('internal_id'));
            let pipeline_connectors = JSON.parse(e.dataTransfer.getData('pipeline_connectors')); 

            //generate snippet in backend, getting new snippet uuid
            invoke('new_snippet', {
                windowSessionUuid: window_session_id,
                externalSnippetUuid: internal_id
            }).then((result) => {
                let snippet_id = result;

                //create drawable snippet
                let snippetDrawable = generateSnippet(snippet_id, name, visualComponents, e.clientX - window_x, e.clientY - window_y, pipeline_connectors, spawnPipeline, snippetDragStart, snippetDragEnd);

                //create snippet
                snippetComponents.push({id: id, name: "testing snippet", type: type, internal_id: internal_id, pipeline_connectors: pipeline_connectors, drawable: snippetDrawable});

                //draw snippet
                drawSnippet(snippetDrawable);
            });
        }
    }

    //draws snippet
    function drawSnippet(snippetDrawable) {
        snippetLayer.add(snippetDrawable);
        //stage.draw();
    }

    //-------handle pipeline creation event--------
    //if a pipeline is in the process of being created
    var pipelineInCreationEvent = null;

    function spawnPipeline(visual_id, position_offset) {
        //if a pipeline is curerntly not being creatd:ed
        if (!pipelineInCreationEvent) {
            //get the visual component from the map
            var pipeline_from_connector = visualComponents[visual_id].visual;
            //check if connector already has pipeline attached (assuming one-to-one policy for now, will be changed in future)
                //return

            //get background rect
            var background_rect = getChild(pipeline_from_connector, "background_rect");

            //get background rect position in canvas space
            var background_rect_position = background_rect.getAbsolutePosition(stage);
            
            //get temp id
            invoke('get_id')
                .then((result) => {
                    let id = result;

                    //start pipeline creation
                    pipelineInCreationEvent = {
                        visual_component: generatePipeConnector(id, visualComponents, background_rect_position.x + position_offset.x, background_rect_position.y + position_offset.y, deletePipeline),
                        pipeline_connector_id: visual_id,
                        start_pos: {
                            x: background_rect_position.x + position_offset.x,
                            y: background_rect_position.y + position_offset.y
                        } 
                    };

                    //draw pipeline
                    pipelineLayer.add(pipelineInCreationEvent.visual_component);
                    stage.draw();

                    //change pipe insert color
                    visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.color = visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.highlight_color;
                });
       }
        else {
            //get virtual components from visual mapping
            var pipeline_from_connector = visualComponents[pipelineInCreationEvent.pipeline_connector_id].visual;
            var pipeline_to_connector= visualComponents[visual_id].visual;

            //if parent pipeline id is the same, cancel
            var pipeline_connector_from_parent = pipeline_from_connector.getParent();
            var pipeline_connector_to_parent = pipeline_to_connector.getParent();

            if (pipeline_connector_from_parent && pipeline_connector_to_parent && pipeline_connector_from_parent.getId() === pipeline_connector_to_parent.getId()) {
                //delete pipeline
                deletePipeline();
                //clear color highlight
                visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.color = visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.default_color;
                return;
            } 

            //if before pipeline the same, then cancel 
            if (pipelineInCreationEvent.visual_component.getId() === pipeline_from_connector.getId()) {
                //delete pipeline     
                deletePipeline();
                //clear color highlight
                visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.color = visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.default_color;
                return;
            }

            //else, try to connect
                //validate connection
                //change colors of pipelines
                /*pipeline_from_connector.getChildren(function(node) {
                    return node.getId() === "background_rect";
                })[0].fill('#fcd777');
                pipelineInCreationEvent.visualcomponent.fill('#fcd777');*/
 
                //create pipeline from backend, get new pipeline id
                let pipeline_id = 0;

                //change id of virtual pipeline              
                pipelineInCreationEvent.visual_component.setId(pipeline_id);

                //get current pipeline connector position
                var background_rect = pipeline_to_connector.getChildren(function(node) {
                    return node.getId() === "background_rect";
                })[0];

                var background_rect_position = background_rect.getAbsolutePosition(stage);

                //correct final position of pipeline
                pipelineInCreationEvent.visual_component.points([
                    0, 
                    0, 
                    background_rect_position.x + position_offset.x - pipelineInCreationEvent.start_pos.x, 
                    background_rect_position.y + position_offset.y - pipelineInCreationEvent.start_pos.y
                ]);

                //add pipeline to visual component mapping
                visualComponents[pipelineInCreationEvent.visual_component.getId()] = 
                {
                    visual: pipelineInCreationEvent.visual_component
                };
                
                //change both ends to connected color
                visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.color = visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.connected_color;
                visualComponents[visual_id].state.color = visualComponents[visual_id].state.connected_color;

                let pipeline_from_connector_background_rect = getChild(pipeline_from_connector, "background_rect");
                let pipeline_to_connector_background_rect = getChild(pipeline_to_connector, "background_rect");

                //draw color change
                pipeline_from_connector_background_rect.fill(visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.color);
                pipeline_to_connector_background_rect.fill(visualComponents[visual_id].state.color);

                //clear event
                pipelineInCreationEvent = null;
            //else, if the same, cancel pipeline connection
        }
    }

    //handle delete pipeline event from pipeline double click
    function deletePipeline() {
        pipelineInCreationEvent.visual_component.destroy();
        pipelineInCreationEvent = null;    
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
            /*pipelineInCreationEvent.visual_component.points([
                0, 
                0, 
                e.clientX - window_x - pipelineInCreationEvent.start_pos.x - stage.x(), 
                e.clientY - window_y - pipelineInCreationEvent.start_pos.y - stage.y()
            ]);

            stage.draw();*/
            //redraw pipeline to connect to where mouse is
        }
        else if (snippetDragEvent) {
            //get mouse displacement from start position
            
            //apply to all ends of pipelines
        }

    }

    function handleCanvasClick(e) {
        if (pipelineInCreationEvent) {
            //if the stage is clicked
            //change to not in hashmap
            //e.target.attrs.id === "stage"
            if (!(e.target instanceof Konva.Shape)) {
                //cancel pipeline creation
                visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.color = visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.default_color;
                //get background rect
                let background_rect = getChild(visualComponents[pipelineInCreationEvent.pipeline_connector_id].visual, "background_rect");

                //fill with changed color (i.e. default)
                background_rect.fill(visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.color);

                pipelineInCreationEvent.visual_component.destroy();
                pipelineInCreationEvent = null;
            }
        }
        //pipeline is in creation and stage is clickenent
    }

    //change to when click on, its orange, and then other color changes to orange too when pipe made
    //when snippet is moving, pipes turn into something else, simpler, like partially transparent dotted lines

    //implement grid lock, and grid background maybe?

    //prevent snippet overlap, ihave list of rectangles that are fit for the grid space they are in! then check for overlap
    //only check on mouse release, when placing it, so on dropend
    //in the session manager, have a virtualgridspace manager that checks for colissions

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