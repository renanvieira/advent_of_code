import { readFileSync } from "fs";
import * as path from 'path';

const raw_input = readFileSync(path.join(__dirname, "input.txt"), 'utf8');

const input = raw_input.split('\n');


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

let prioritySum = 0;

for (const zack of input) {

    const half = Math.trunc(zack.length / 2)
    const firstCompartiment = new Set(zack.slice(0, half))
    const secondCompartiment = new Set(zack.slice(half, zack.length))

    const intersection = new Set<string>([...firstCompartiment].filter(x => secondCompartiment.has(x)))

    for (const letter of intersection) {
        prioritySum += itemsPriority[letter]
    }


}

console.log(`Sum: ${prioritySum}`)
