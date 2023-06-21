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
const scenicScoreMap = new Map<string, Array<number>>();

while (selectedTree.x <= lastColIndex && selectedTree.y <= lastRowIndex) {
    // Check if all trees until the edge are < selected;    
    const selectedValue = florest[selectedTree.y][selectedTree.x];

    // console.log(`- Selected Tree: ${selectedValue} @ ${selectedTree.getPoints()}`);

    for (let i = 0; i < directionsArray.length; i++) {
        const direction = directionsArray[i];
        let scenirScoreDirection = 0;

        navigationPointer = new Point(selectedTree.x, selectedTree.y);
        navigationPointer.move(direction);

        // console.log(`--- Nav. Pointer: ${navigationPointer.getPoints()}`);
        // console.log(`--- Is Valid: ${isNavPointerValid}`);

        while (navigationPointer.isValid(maxRows, maxCols, direction)) {
            // console.log(` ---- Nav. Pointer (mov): ${navigationPointer.getPoints()}`);
            const navPointerValue = florest[navigationPointer.y][navigationPointer.x];
            // console.log(` ---- Nav. Value (mov): ${navPointerValue} @ ${navigationPointer.getPoints()}`);

            if (selectedValue > navPointerValue) {
                scenirScoreDirection += 1;
            } else {
                scenirScoreDirection += 1;
                // console.log(` ---- Blocked Tree`);
                break;
            }

            navigationPointer.move(direction);
        }

        if (scenicScoreMap.has(selectedTree.getPoints())) {
            scenicScoreMap.get(selectedTree.getPoints())?.push(scenirScoreDirection);
        } else {
            scenicScoreMap.set(selectedTree.getPoints(), [scenirScoreDirection]);
        }
    }

    selectedTree.move(Directions.RIGHT);
    if (selectedTree.isValid(maxRows, maxCols, Directions.RIGHT) === false) {
        selectedTree = new Point(0, selectedTree.y + 1)
    }
    // console.log("-----------------------------------------------------------")
}

let scenicScoreSum = [];

for (let scenicScores of scenicScoreMap.values()) {
    scenicScoreSum.push(scenicScores.reduce((acc, curr) => acc * curr));
}

scenicScoreSum = scenicScoreSum.sort((a, b) => a - b).reverse();

console.log(`Consider each tree on your map. What is the highest scenic score possible for any tree? ${scenicScoreSum[0]}`)
