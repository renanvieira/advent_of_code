
import readInput from "../Utils/input";
import { CargoBayPart2 } from "./CargoBay";
import { Stack } from "./Stack";

const input = readInput("input.txt")

const raw_crates = input.slice(0, input.indexOf("")).slice(0, -1).reverse()

const operations = input.slice(input.indexOf("") + 1, input.length)

const total_crates = Math.ceil(raw_crates[0].length / 4)

const initial_map = Array.from({ length: total_crates }, _ => new Stack<string>());


for (let i = 0; i < raw_crates.length; i++) {

    for (let j = 1; j <= total_crates; j++) {

        const end = j * 4;
        const start = end - 4;


        const crate = raw_crates[i].slice(start, end).trim()
        if (crate) {
            initial_map[j - 1].push([crate]);
        }
    }
}
const cargo_bay = new CargoBayPart2(initial_map); // Part 2 behaviour change

for (const op of operations) {
    const matches = op.match(/\d+/g);

    if (matches) {
        const quantity = Number(matches[0]);
        const source_index = Number(matches[1]) - 1;
        const destination_index = Number(matches[2]) - 1;

        cargo_bay.moveContainer(source_index, destination_index, quantity);


    }
}


console.log(`After the rearrangement procedure completes, what crate ends up on top of each stack? ${cargo_bay.getTopItemsFromStacks().join('').replaceAll(']','').replaceAll('[', '')}`);

