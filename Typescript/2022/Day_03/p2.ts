
import { readFileSync } from "fs";
import * as path from 'path';

const raw_input = readFileSync(path.join(__dirname, "input.txt"), 'utf8');

const input = raw_input.split('\n');

// Build the item priority list
const itemsPriority: { [key: string]: number } = {}

for (let i = 0; i < 52; i++) {

    let letter: string;

    if (i < 26) { // lowercase
        letter = String.fromCharCode(97 + i);
    } else { // uppercase
        letter = String.fromCharCode(65 + i - 26);
    }

    itemsPriority[letter] = i + 1
}

// Group rugzacks in groups of 3
let group: Array<Array<Set<string>>> = []
let aux = [new Set(input[0].split(''))]

for (let i = 1; i < input.length; i++) {

    if (i % 3 == 0) {
        group.push(aux)
        aux = []
    }


    aux.push(new Set(input[i].split('')))
}

let prioritySum: number = 0

for (const g of group) {

    const common = [...g[0]].filter(char => g.every(array => array.has(char)))

    for (const badge of common) {
        prioritySum += itemsPriority[badge]
    }
}

console.log(`The Sum is: ${prioritySum}`)

