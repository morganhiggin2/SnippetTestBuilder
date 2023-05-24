import Konva from 'konva';
import { invoke } from '@tauri-apps/api';

export function generateSnippet(id, name, visualComponents, xPos, yPos, pipeline_connectors, spawnPipeline, deleteSnippet, dragStart, dragEnd) {
    // make the snippet 
    var snippet_group = new Konva.Group({
        id: id,
        draggable: true 
    });

    //the title text
    var titleText = new Konva.Text({
        id: "title_text",
        x: xPos,
        y: yPos + 2,
        text: name,
        fontSize: 16,
        fontFamily: 'Inter',
        fill: 'white'
    });

    //get dimensions of title text
    let textWidth = titleText.getWidth();
    let textHeight = titleText.getHeight();

    //create pipe inserts
    var leftPipeInserts = [];
    var rightPipeInserts = [];

    //go though pipelines, with assigning id
    for (var i = 0; i < pipeline_connectors.length; i++) {
        if (pipeline_connectors[i].input) {
            leftPipeInserts.push(createPipeInsert(pipeline_connectors[i].id, visualComponents, pipeline_connectors[i].name, xPos, yPos + textHeight + 8, true, spawnPipeline));
        }
        else {
            rightPipeInserts.push(createPipeInsert(pipeline_connectors[i].id, visualComponents, pipeline_connectors[i].name, xPos, yPos + textHeight + 8, false, spawnPipeline));
        }
    }

    //get needed measurements
    //get the max width of the left and right pipe inserts
    var leftPipeInsertsWidth = Math.max.apply(Math, leftPipeInserts.map(pipeInsert => pipeInsert.width));
    var rightPipeInsertsWidth = Math.max.apply(Math, rightPipeInserts.map(pipeInsert => pipeInsert.width));
    var pipeInsertsHeight = Math.max(
        leftPipeInserts.map(pipeInsert => pipeInsert.height).reduce((partSum, a) => partSum + a + 4, 0),
        rightPipeInserts.map(pipeInsert => pipeInsert.height).reduce((partSum, a) => partSum + a + 4, 0)
    );
    
    //get rect width
    let rectWidth = Math.max(textWidth + 40, leftPipeInsertsWidth + 4 + rightPipeInsertsWidth, 20);
    let rectHeight = textHeight + 4 + pipeInsertsHeight + 4;

    {
        var yDisplacement = 0;
        
        //move left pipe inserts down accordingly
        for (var i = 0; i < leftPipeInserts.length; i++) {
            leftPipeInserts[i].pipe.y(yDisplacement);

            yDisplacement += leftPipeInserts[0].height + 4;
        }
    }

    {
        var yDisplacement = 0;
        
        //move left pipe inserts down accordingly and to the right based on left pipe width
        for (var i = 0; i < rightPipeInserts.length; i++) {
            rightPipeInserts[i].pipe.y(yDisplacement);
            rightPipeInserts[i].pipe.x(rectWidth);

            yDisplacement += rightPipeInserts[0].height + 2;
        }
    }

    //claculate position of title text

    titleText.setPosition({x: xPos + (rectWidth - textWidth) / 2, y: titleText.getPosition().y});

    //main rectangle
    var backgroundRect = new Konva.Rect({
        id: "background_rect",
        x: xPos,
        y: yPos,
        width: rectWidth,
        height: rectHeight,
        fill: '#ededed',
        cornerRadius: 3,
        shadowColor: 'black',
        shadowBlur: 2,
        shadowOffset: { x: 2, y: 2 },
        shadowOpacity: 0.2,
        draggable: false
    }); 

    var titleBackgroundRect = new Konva.Rect({
        id: "title_backgrond_rect",
        x: xPos,
        y: yPos,
        width: rectWidth,
        height: textHeight + 3,
        fill: '#31abf5',
        cornerRadius: [3, 3, 0, 0],
    });

    var titleSeperatorLine = new Konva.Line({
        id: "title_seperator_line",
        x: xPos,
        y: yPos + textHeight + 3,
        points: [0, 0, rectWidth, 0],
        stroke: '#0070b3',
        strokeWidth: 2
    });


    //snippet events
    titleBackgroundRect.on('dblclick', () => {deleteSnippet(id)});
    backgroundRect.on('dragstart', () => {dragStart(id)});
    backgroundRect.on('dragend', () => {dragEnd(id)});
    //singlePipeInsert.pipe.on('click', () => {});

    snippet_group.add(backgroundRect);
    snippet_group.add(titleBackgroundRect);
    snippet_group.add(titleSeperatorLine);
    snippet_group.add(titleText);

    //add pipe inserts to stage and visualComponents
    for (var i = 0; i < leftPipeInserts.length; i++) {
        snippet_group.add(leftPipeInserts[i].pipe);
    }

    for (var i = 0; i < rightPipeInserts.length; i++) {
        snippet_group.add(rightPipeInserts[i].pipe);
    }
    //snippet_group.add(singlePipeInsert.pipe);

    //add all visually linked components to visualComponents map
    visualComponents[id] = {
        visual: snippet_group,
        type: "snippet"
    };

    return snippet_group;

    //todo array pipe insert where its single pipe insert with + on bottom, and when another pipe gets added it expands to add one more pipe insert 
    //on top and moves the + down, this will be its own type, multiplePipeInsert
}

export function generatePipeConnector(id, visualComponents, x_pos_start, y_pos_start, x_end_offset, y_end_offset, deletePipeline) {
    var line = new Konva.Line({
        id: id,
        x: x_pos_start,
        y: y_pos_start,
        points: [0, 0, x_end_offset, y_end_offset],
        stroke: '#fcd777',
        tension: 0,
        strokeWidth: 6
    });

    line.on('dblclick', () => {deletePipeline(id)}); //deletePipeline(line)

    //add visually linked component to map
    visualComponents[id] = {
        visual: line,
        type: "pipe"
    };

    return line;
}

function createPipeInsert(id, visualComponents, name, xPos, yPos, left = false, spawnPipeline) {
    //create group for pipe
    var pipeGroup = new Konva.Group({
        id: id
    });

    //crete text next to pipe insert
    var titleText = new Konva.Text({
        id: "title_text",
        x: xPos,
        y: yPos,
        text: name,
        fontSize: 12,
        fontFamily: 'Inter',
        fill: 'black'
    });

    //get the width of the text
    var titleTextWidth = titleText.getWidth();

    //set title position to offset from pipe insert
    if (left) {
        titleText.setPosition({x: xPos + 10, y: yPos + (14 - titleText.getHeight()) / 2});
    }
    else {
        titleText.setPosition({x: xPos - titleTextWidth - 10, y: yPos + (14 - titleText.getHeight()) / 2});
    }

    //set the background position
    var backgroundRectPosition = {x: 0, y:0};
    var backgroundRectCorners = [0, 0, 0, 0];

    //for pipeline start point
    var pipelineConnectorPositionOffset = {x: 0, y: 0};

    if (left) {
        backgroundRectPosition = {
            x: xPos,
            y: yPos
        };

        backgroundRectCorners = [0, 2, 2, 0];

        pipelineConnectorPositionOffset = {
            x: 0,
            y: 7 //half the height
        };
    }
    else {
        backgroundRectPosition = {
            x: xPos - 8,
            y: yPos
        };

        backgroundRectCorners = [2, 0, 0, 2];

        pipelineConnectorPositionOffset = {
            x: 8, //the pipeline starts on the right side
            y: 7 //half the height
        }
    }

    //create background rect for pipe
    var backgroundRect = new Konva.Rect({
        id: "background_rect",
        x: backgroundRectPosition.x, 
        y: backgroundRectPosition.y,
        width: 8,
        height: 14,
        fill: '#a1a1a1',
        cornerRadius: backgroundRectCorners,
        draggable: false
    }); 
    //add elements to group
    pipeGroup.add(backgroundRect);
    pipeGroup.add(titleText);
    
    visualComponents[id] = {
            visual: pipeGroup,
            type: "pipe_insert",
            state: {
                color: '#a1a1a1',
                highlight_color: '#fcd777',
                default_color: '#a1a1a1',
                connected_color: '#fcd777'
            }
        };

    //set events such going over pipe insert selects it
    backgroundRect.on('mouseover', () => {backgroundRect.fill(visualComponents[id].state.highlight_color)});
    backgroundRect.on('mouseout', () => {backgroundRect.fill(visualComponents[id].state.color)});
    backgroundRect.on('click', () => {spawnPipeline(id, pipelineConnectorPositionOffset)});

    //calculate dimensions
    let totalWidth = titleTextWidth + 4 + 8;
    let totalHeight = 14;

    //return pipe and dimensions
    return {
        pipe: pipeGroup,
        width: totalWidth,
        height: totalHeight
    };
}

export function getChild(node, id) {
    return node.getChildren(function (node) {
        return node.getId() === id;
    })[0];
}