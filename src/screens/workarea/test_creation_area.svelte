<script>
    import {invoke} from '@tauri-apps/api';
    import { onMount } from 'svelte';
    import { generatePipeConnector, generateSnippet, getChild } from './snippet_module.js';
    import Konva from 'konva';

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
    //let snippetComponents = [];

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
            let internal_id = JSON.parse(e.dataTransfer.getData('internal_id'));

            //TODO handle new snippet front information
            //generate snippet in backend, getting new snippet uuid
            invoke('new_snippet', {
                windowSessionUuid: window_session_id,
                externalSnippetUuid: internal_id
            }).then((result) => {
                let snippet_id = result;

                //get snippet information
                invoke('get_snippet_information', {
                    windowSessionUuid: window_session_id,
                    snippetUuid: snippet_id
                })
                .then((result) => {
                    //snippet information
                    let snippet_information = result;

                    //create drawable snippet
                    let snippetDrawable = generateSnippet(snippet_id, snippet_information.internal_id, snippet_information.name, visualComponents, e.clientX - window_x, e.clientY - window_y, snippet_information.pipeline_connectors, spawnPipeline, snippetDragStart, snippetDragEnd);

                    //create snippet
                    //snippetComponents.push({id: snippet_information.id, name: snippet_information.name, internal_id: snippet_id, pipeline_connectors: snippet_information.pipeline_connectors, drawable: snippetDrawable});

                    //draw snippet
                    drawSnippet(snippetDrawable);
                })
                .catch((e) => {
                    invoke('logln', {text: JSON.stringify(e)});
                });
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

    function spawnPipeline(other_pipeline_connector_id, position_offset) {
        //get the visual component from the map
        var pipeline_from_connector = visualComponents[other_pipeline_connector_id];

        //check to see if pipeline connector capacity is already full
        invoke('check_pipeline_connector_capacity_full', {
            windowSessionUuid: window_session_id,
            pipelineConnectorUuid: pipeline_from_connector.internal_id
        })
        .then((result) => {
            let capacity_full = result;
            
            //if a pipeline is curerntly not being created
            //and capacity is not full
            if (!capacity_full) {
                if (!pipelineInCreationEvent) {
                    //get the visual component from the map
                    var pipeline_from_connector = visualComponents[other_pipeline_connector_id];
                    //check if connector already has pipeline attached (assuming one-to-one policy for now, will be changed in future)
                        //return

                    //get background rect
                    var background_rect = getChild(pipeline_from_connector.visual, "background_rect");

                    //get background rect position in canvas space
                    var background_rect_position = background_rect.getAbsolutePosition(stage);

                    //start pipeline creation
                    pipelineInCreationEvent = {
                        //DELvisual_component: generatePipeConnector(id, visualComponents, background_rect_position.x + position_offset.x, background_rect_position.y + position_offset.y, deletePipeline),
                        pipeline_connector_id: other_pipeline_connector_id,
                        start_pos: {
                            x: background_rect_position.x + position_offset.x,
                            y: background_rect_position.y + position_offset.y
                        }
                    };

                    //draw pipeline
                    //DEL
                    //change pipe insert color
                    visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.color = visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.highlight_color;
                }
                else {
                    //get virtual components from visual mapping
                    var pipeline_from_connector = visualComponents[pipelineInCreationEvent.pipeline_connector_id];
                    var pipeline_to_connector= visualComponents[other_pipeline_connector_id];

                    //validate pipeline connection
                    invoke('validate_pipeline_connection', {windowSessionUuid: window_session_id, fromUuid: pipeline_from_connector.internal_id, toUuid: pipeline_to_connector.internal_id})
                    .then((result) => {
                        let validated = result;
                        
                        //if it fails validation
                        if (!validated) {
                            //change from and to colors back
                            visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.color = visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.default_color;
                            visualComponents[other_pipeline_connector_id].state.color = visualComponents[other_pipeline_connector_id].state.default_color;

                            let pipeline_from_connector_background_rect = getChild(pipeline_from_connector.visual, "background_rect");
                            let pipeline_to_connector_background_rect = getChild(pipeline_to_connector.visual, "background_rect");

                            //draw color change
                            pipeline_from_connector_background_rect.fill(visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.color);
                            pipeline_to_connector_background_rect.fill(visualComponents[other_pipeline_connector_id].state.color);

                            //delete pipeline event
                            pipelineInCreationEvent = null;

                            return;
                        }

                        //create pipeline in backend
                        invoke('new_pipeline', {
                            windowSessionUuid: window_session_id,
                            fromUuid: pipeline_from_connector.internal_id,
                            toUuid: pipeline_to_connector.internal_id,
                        })
                        .then((result) => {
                            //TODO have new_pipieline return front result with id and internal_id
                            //get pipeline uuid in backend
                            let pipeline_uuid = result;
                            
                            //generate uuid for pipeline visual id
                            invoke('get_id', {})
                            .then((result) => {
                                let pipeline_id = result;

                                //set pipeline uuid for visual component
                                var to_background_rect = pipeline_to_connector.visual.getChildren(function(node) {
                                    return node.getId() === "background_rect";
                                })[0];

                                var to_background_rect_position = to_background_rect.getAbsolutePosition(stage);

                                //create visual pipeline
                                var visual_component = generatePipeConnector(pipeline_id, visualComponents, pipelineInCreationEvent.start_pos.x, pipelineInCreationEvent.start_pos.y, to_background_rect_position.x + position_offset.x - pipelineInCreationEvent.start_pos.x, to_background_rect_position.y + position_offset.y - pipelineInCreationEvent.start_pos.y, deletePipeline);
                                
                                //add to pipeline layers
                                pipelineLayer.add(visual_component);

                                //add pipeline to visual component mapping
                                visualComponents[pipeline_id] = 
                                {
                                    visual: visual_component,
                                    type: "pipeline",
                                    internal_id: pipeline_uuid
                                };
                                
                                //change both ends to connected color
                                visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.color = visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.connected_color;
                                visualComponents[other_pipeline_connector_id].state.color = visualComponents[other_pipeline_connector_id].state.connected_color;

                                let pipeline_from_connector_background_rect = getChild(pipeline_from_connector.visual, "background_rect");
                                let pipeline_to_connector_background_rect = getChild(pipeline_to_connector.visual, "background_rect");

                                //draw color change
                                pipeline_from_connector_background_rect.fill(visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.color);
                                pipeline_to_connector_background_rect.fill(visualComponents[other_pipeline_connector_id].state.color);

                                //clear event
                                pipelineInCreationEvent = null;
    
                            })
                            .catch((e) => {
                                invoke('logln', {text: JSON.stringify(e)});
                            });
                        })
                        .catch((e) => {
                            invoke('logln', {text: JSON.stringify(e)})
                        })
                        //invoke 
                        
                    })
                    .catch((e) => {
                        invoke('logln', {text: JSON.stringify(e)});
                    });
                }
            }           
            //else, do nothing (don't allow it)
        })
        .catch((e) => {
            invoke('logln', {text: JSON.stringify(e)});
        });
    }

    //handle delete pipeline event from pipeline double click
    function deletePipeline(id) {
        pipelineInCreationEvent = null;    
        
        //get pipeline
        let pipeline = visualComponents[id];

        //get pipeline connectors

        //pipelineInCreationEvent.visual_component.destroy();                    

        //change from and to colors back
        //visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.color = visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.default_color;
        //visualComponents[other_pipeline_connector_id].state.color = visualComponents[other_pipeline_connector_id].state.default_color;


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

                //DELpipelineInCreationEvent.visual_component.destroy();
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