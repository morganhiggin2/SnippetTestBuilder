<script>
    import {invoke} from '@tauri-apps/api';
    import { onMount } from 'svelte';
    import { generatePipeConnector, generateSnippet, getChild, setNewPositionPipeConnector, getPipelineConnectorPositionOffset} from './snippet_module.js';
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

    // parameters methods
    export let add_parameters;
    export let delete_parameters;

    onMount(async () =>
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

    async function handleDrop(e) {
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
            let directory_id = JSON.parse(e.dataTransfer.getData('_id'));

            //generate snippet in backend, getting new snippet information 
            let snippet_information = null;

            try {
                snippet_information = await invoke('new_snippet', {
                    windowSessionUuid: window_session_id,
                    directoryFrontUuid: directory_id
                });

                // add parameters to parameter screen
                // add parameter to parameters list based on snippet id
                add_parameters(snippet_information.id, snippet_information.parameters)

            } catch (e) {
                invoke('logln', {text: JSON.stringify(e)});
                return;
            }

            //get stage dragged offset
            let stage_drag_offset = stage.absolutePosition();

            //create drawable snippet
            let snippetDrawable = generateSnippet(snippet_information.id, snippet_information.name, visualComponents, e.clientX - window_x - stage_drag_offset.x, e.clientY - window_y - stage_drag_offset.y, snippet_information.pipeline_connectors, spawnPipeline, deleteSnippet, snippetDragStart, snippetDragEnd);

            //create snippet
            //snippetComponents.push({id: snippet_information.id, name: snippet_information.name, internal_id: snippet_id, pipeline_connectors: snippet_information.pipeline_connectors, drawable: snippetDrawable});


            //draw snippet
            drawSnippet(snippetDrawable);
        }
    }

    async function deleteSnippet(id) {
        //get visual component
        let snippet_component = visualComponents[id];

        //get all pipelines associated with snippet
        var result;

        try {
            result = await invoke('get_snippet_pipelines', {
                windowSessionUuid: window_session_id,
                snippetFrontUuid: id 
            });
        } catch (e) {
            invoke('logln', {text: JSON.stringify(e)});
            return;
        }

        var pipelinesUuid = result;

        //delete pipelines associated with snippet
        for (var i = 0; i < pipelinesUuid.length; i++) {
            let pipelineUuid = pipelinesUuid[i];

            //get pipeline connectors for each pipeline
            try {
                result = await invoke('get_pipeline_connector_uuids_from_pipeline', {
                    windowSessionUuid: window_session_id,
                    frontPipelineUuid: pipelineUuid 
                });

            } catch (e) {
                invoke('logln', {text: JSON.stringify(e)});
            } 

            //change color in visual components
            //get pipeline connectors
            let pipelineConnectorUuids = result;

            let from_pipeline_connector = visualComponents[pipelineConnectorUuids.front_from_pipeline_connector_uuid];
            let to_pipeline_connector = visualComponents[pipelineConnectorUuids.front_to_pipeline_connector_uuid];
 

            //change both ends to connected color
            from_pipeline_connector.state.color = from_pipeline_connector.state.default_color;
            to_pipeline_connector.state.color = to_pipeline_connector.state.default_color;

            let pipeline_from_connector_background_rect = getChild(from_pipeline_connector.visual, "background_rect");
            let pipeline_to_connector_background_rect = getChild(to_pipeline_connector.visual, "background_rect");

            //draw color change
            pipeline_from_connector_background_rect.fill(from_pipeline_connector.state.color);
            pipeline_to_connector_background_rect.fill(to_pipeline_connector.state.color);

            //destory pipeline in stage
            var pipeline = visualComponents[pipelineUuid];
            pipeline.visual.destroy();

            //remove pipeline from visual components
            delete visualComponents[pipelineUuid];
            
            //delete pipeline in backend
            try {
                result = await invoke('delete_pipeline', {
                    windowSessionUuid: window_session_id,
                    frontUuid: pipelineUuid 
                });

            } catch (e) {
                invoke('logln', {text: JSON.stringify(e)});
            } 
        }

        //remove visual component from stage
        snippet_component.visual.destroy();

        //remove snippet from visual components
        delete visualComponents[id];

        // delete snippet parameters 
        delete_parameters(id); 

        //delete snippet in backend
        try {
            result = await invoke('delete_snippet', {
                windowSessionUuid: window_session_id,
                frontUuid: id
            });
        } catch (e) {
            invoke('logln', {text: JSON.stringify(e)});
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

    async function spawnPipeline(other_pipeline_connector_id, position_offset) {
        //get the visual component from the map
        var pipeline_from_connector = visualComponents[other_pipeline_connector_id];

        let capacity_full = true;

        //check to see if pipeline connector capacity is already full
        try {
            capacity_full = await invoke('check_pipeline_connector_capacity_full', {
                windowSessionUuid: window_session_id,
                frontPipelineConnectorUuid: other_pipeline_connector_id
            });
        } catch(e) {
            invoke('logln', {text: JSON.stringify(e)})
        }

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

                //change pipe insert color
                visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.color = visualComponents[pipelineInCreationEvent.pipeline_connector_id].state.highlight_color;
            }
            else {
                //get virtual components from visual mapping
                var pipeline_from_connector = visualComponents[pipelineInCreationEvent.pipeline_connector_id];
                var pipeline_to_connector= visualComponents[other_pipeline_connector_id];

                let validated = false;

                //validate pipeline connection
                try {
                    validated = await invoke('validate_pipeline_connection', {
                        windowSessionUuid: window_session_id, 
                        fromFrontUuid: pipelineInCreationEvent.pipeline_connector_id, 
                        toFrontUuid: other_pipeline_connector_id 
                    });
                } catch(e) {
                    invoke('logln', {text: JSON.stringify(e)})
                }
            
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
                
                let pipeline_uuid = 0;
                let pipeline_id = 0; 

                //create pipeline in backend
                try {
                    let result = await invoke('new_pipeline', {
                        windowSessionUuid: window_session_id,
                        fromFrontUuid: pipelineInCreationEvent.pipeline_connector_id,
                        toFrontUuid: other_pipeline_connector_id,
                    });

                    pipeline_uuid = result.iid;
                    pipeline_id = result.id;
 
                } catch (e) {
                    invoke('logln', {text: JSON.stringify(e)});
                }
            
                //get pipeline uuid in backend
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
        }
        } 
    }

    //handle delete pipeline event from pipeline double click
    async function deletePipeline(id) {
        pipelineInCreationEvent = null;    
        
        //get pipeline
        let pipeline = visualComponents[id];

        //result from invoke
        let result = null;

        //get pipeline connectors
        try {
            result = await invoke('get_pipeline_connector_uuids_from_pipeline', {
                windowSessionUuid: window_session_id,
                frontPipelineUuid: id
            });

        } catch (e) {
            invoke('logln', {text: JSON.stringify(e)});
        } 

        //extract pipeline connector ids
        let from_pipeline_connector_id = result.front_from_pipeline_connector_uuid;
        let to_pipeline_connector_id = result.front_to_pipeline_connector_uuid;

        //delete pipeline in backend
        try {
            result = await invoke('delete_pipeline', {
                windowSessionUuid: window_session_id,
                frontUuid: id
            });

        } catch (e) {
            invoke('logln', {text: JSON.stringify(e)});
        } 

        var pipeline_from_connector = visualComponents[from_pipeline_connector_id];
        var pipeline_to_connector = visualComponents[to_pipeline_connector_id];

        //change from and to colors back
        pipeline_from_connector.state.color = pipeline_from_connector.state.default_color;
        pipeline_to_connector.state.color = pipeline_to_connector.state.default_color;


        let pipeline_from_connector_background_rect = getChild(pipeline_from_connector.visual, "background_rect");
        let pipeline_to_connector_background_rect = getChild(pipeline_to_connector.visual, "background_rect");

        //draw color change
        pipeline_from_connector_background_rect.fill(pipeline_from_connector.state.color);
        pipeline_to_connector_background_rect.fill(pipeline_to_connector.state.color);

        //destroy pipeline visual component
        pipeline.visual.destroy();                    

        //remove from visual components
        delete visualComponents[pipeline];
 
        stage.draw();
    }

    //------snippet drag event---------
    let snippetDragEvent = null;

    //for snippet drag event
    async function snippetDragStart(id) {
        //get all from_uuids and to_uuids of snippet
        //get all pipeline uuids associated with these (call to backend)

        //get visual component
        let snippet_component = visualComponents[id];

        //get all pipelines associated with snippet
        var result;

        try {
            result = await invoke('get_snippet_pipelines', {
                windowSessionUuid: window_session_id,
                snippetFrontUuid: id 
            });
        } catch (e) {
            invoke('logln', {text: JSON.stringify(e)});
            return;
        }

        var pipelinesUuid = result;

        //delete pipelines associated with snippet
        for (var i = 0; i < pipelinesUuid.length; i++) {
            let pipelineUuid = pipelinesUuid[i];

            //make pipelines disapear
            visualComponents[pipelineUuid].visual.hide();
        }
        
        stage.draw();

        //create snippet drag start event
        snippetDragEvent = {
            snippet_id: id,
            pipelines_uuid: pipelinesUuid
        };
    }

    async function snippetDragEnd(id) {
        if (!snippetDragEvent) {
            invoke("logln", {text: "snippet drag end event is null, should have some value"});
        }

        var result;

        //get pipelines uuids from snippet drag event
        let pipelinesUuid = snippetDragEvent.pipelines_uuid;

        //get stage dragged offset
        let stage_drag_offset = stage.absolutePosition();

        //reposition all pipelines (both start and end)
        for (var i = 0; i < pipelinesUuid.length; i++) {
            let pipelineUuid = pipelinesUuid[i];

            //get pipeline connectors for each pipeline
            try {
                result = await invoke('get_pipeline_connector_uuids_from_pipeline', {
                    windowSessionUuid: window_session_id,
                    frontPipelineUuid: pipelineUuid 
                });

            } catch (e) {
                invoke('logln', {text: JSON.stringify(e)});
            } 

            //extract pipeline connector ids
            let from_pipeline_connector_id = result.front_from_pipeline_connector_uuid;
            let to_pipeline_connector_id = result.front_to_pipeline_connector_uuid;


            let to_pipeline_connector = visualComponents[to_pipeline_connector_id];
            let from_pipeline_connector = visualComponents[from_pipeline_connector_id];

            //get positions
            var to_background_rect = to_pipeline_connector.visual.getChildren(function(node) {
                return node.getId() === "background_rect";
            })[0];
            var from_background_rect = from_pipeline_connector.visual.getChildren(function(node) {
                return node.getId() === "background_rect";
            })[0];


            var to_background_rect_position = to_background_rect.getAbsolutePosition(stage);
            var from_background_rect_position = from_background_rect.getAbsolutePosition(stage);
        
            //get weither or not this is a left facing pipeline connector
            var left = to_pipeline_connector.visual.getAttr('left');

            var pipelineConnectorPositionOffset = getPipelineConnectorPositionOffset(left);
      
            //set pipeline position
            setNewPositionPipeConnector(visualComponents[pipelineUuid].visual, from_background_rect_position.x + stage_drag_offset.x, from_background_rect_position.y + stage_drag_offset.y, to_background_rect_position.x - from_background_rect_position.x + pipelineConnectorPositionOffset.x, to_background_rect_position.y - from_background_rect_position.y + pipelineConnectorPositionOffset.y);

            //make pipelines visible
            visualComponents[pipelineUuid].visual.show();

            //remove snippet drag event by setting to null
            snippetDragEvent = null;
        }

        stage.draw();
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
    //only check on mouse release, when placing it, so on dropend          on:dragover|preventDefault={() => {return false;}} 
    //in the session manager, have a virtualgridspace manager that checks for colissions

    //funcationlize / split up code
</script>

<svelte:window bind:innerWidth={window_width} bind:innerHeight={window_height}/>

<div class="body" on:drop|preventDefault={handleDrop} on:dragover|preventDefault on:dragenter|preventDefault on:dragleave|preventDefault on:mousemove={handleMouseMovement} bind:this={selfObj}>
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