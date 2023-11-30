
import { readFileSync } from "fs";


function readInput(fileName: string = "example_input.txt"): Array<string> {
    const raw_input = readFileSync(fileName, 'utf8');

    const input = raw_input.split('\n');

    return input
}


export = readInput;
