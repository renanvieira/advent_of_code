import readInput from "../Utils/input";
import { Directions, Point } from "./Point";

const input = readInput('input.txt');

// 0 = shortest tree;
// 9 = tallest three
// No diagonal lookup

const florest = new Array<Array<number>>();

for (const line of input.slice(0, -1)) {
    const treeArray = Array.from(line).map(t => Number(t));

    florest.push(treeArray);
}

const maxRows = florest.length; // Rows
const maxCols = florest[0].length; // Cols

const lastRowIndex = maxRows - 1;
const lastColIndex = maxCols - 1;

let selectedTree: Point = new Point(0, 0)
let navigationPointer: Point = new Point(0, 0);

const directionsArray = [Directions.RIGHT, Directions.LEFT, Directions.UP, Directions.DOWN];
const visibilityMap = new Map<string, boolean>();


while (selectedTree.x <= lastColIndex && selectedTree.y <= lastRowIndex) {
    // Check if all trees until the edge are < selected;    
    let isTreeVisible = false;
    const selectedValue = florest[selectedTree.y][selectedTree.x];

    // console.log(`- Selected Tree: ${selectedValue} @ ${selectedTree.getPoints()}`);

    for (let i = 0; i < directionsArray.length; i++) {
        const direction = directionsArray[i];

        navigationPointer = new Point(selectedTree.x, selectedTree.y);
        navigationPointer.move(direction);

        const isNavPointerValid = navigationPointer.isValid(maxRows, maxCols, direction);

        // console.log(`--- Nav. Pointer: ${navigationPointer.getPoints()}`);
        // console.log(`--- Is Valid: ${isNavPointerValid}`);

        if (isNavPointerValid === false && !visibilityMap.has(selectedTree.getPoints())) {
            visibilityMap.set(selectedTree.getPoints(), true);
        }

        while (navigationPointer.isValid(maxRows, maxCols, direction)) {
            // console.log(` ---- Nav. Pointer (mov): ${navigationPointer.getPoints()}`);
            const navPointerValue = florest[navigationPointer.y][navigationPointer.x];
            // console.log(` ---- Nav. Value (mov): ${navPointerValue} @ ${navigationPointer.getPoints()}`);

            if (selectedValue > navPointerValue) {
                isTreeVisible = true;
            } else {
                isTreeVisible = false;
                // console.log(` ---- Blocked Tree`);
                break;
            }

            navigationPointer.move(direction);
        }

        if (isTreeVisible && !visibilityMap.get(selectedTree.getPoints())) {
            visibilityMap.set(selectedTree.getPoints(), true);
        }


    }




    selectedTree.move(Directions.RIGHT);
    if (selectedTree.isValid(maxRows, maxCols, Directions.RIGHT) === false) {
        selectedTree = new Point(0, selectedTree.y + 1)
    }
    // console.log("-----------------------------------------------------------")
}


console.log(`Consider each tree on your map. What is the highest scenic score possible for any tree? ${Array.from(visibilityMap.entries()).length}`);

