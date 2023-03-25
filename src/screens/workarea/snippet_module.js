import Konva from 'konva';
import { invoke } from '@tauri-apps/api';

export function generateSnippet(xPos, yPos) {
    var snippet_group = new Konva.Group({});

    var nameText = new Konva.Text({
        x: xPos,
        y: yPos,
        text: 'hello',
        fontSize: 20,
        fontFamily: 'Calibri',
        fill: 'black'
    });

    let textWidth = nameText.getWidth();
    let rectWidth = Math.max(textWidth, 100);

    nameText.setPosition({x: xPos + (rectWidth - textWidth) / 2, y: nameText.getPosition().y});

    var rect = new Konva.Rect({
        x: xPos,
        y: yPos,
        width: rectWidth,
        height: 50,
        fill: 'tan',
        stroke: 'light gray',
        strokeWidth: 2,
        cornerRadius: 4,
        draggable: false
    }); 

    var circle = new Konva.Circle({
        x: xPos + rectWidth,
        y: yPos + 25,
        radius: 5,
        stroke: 'red',
        strokeWidth: 4 
    });

    rect.on('dblclick', () => {});
    circle.on('click', () => {});

    snippet_group.add(rect);
    snippet_group.add(nameText);
    snippet_group.add(circle);

    return snippet_group;
}