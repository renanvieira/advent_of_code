import readInput from "../Utils/input";

const input = readInput('input.txt').slice(0, -1)

let res = new Array<number>(input.length).fill(0)

for (let i = 0; i < input.length; i++) {
    const str_array = Array.from(input[i]);

    for (let j = 0; j < str_array.length; j++) {
        const slide_window = new Set(str_array.slice(j, j + 14));


        if (slide_window.size === 14) {
            res[i] = j + 14;
            break;
        }
    }


}

console.dir(res);
