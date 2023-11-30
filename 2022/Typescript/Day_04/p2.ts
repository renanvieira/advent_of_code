
import readInput from "../Utils/input";

const raw_input = readInput("input.txt")

const input = raw_input.map(pairs => {
    const splitted = pairs.split(',')

    return splitted
}).filter(p => p[0] != '');

let overlappingTask = 0;

for (const group of input) {
    const firstSection = group[0].split('-').map(x => Number(x))
    const secondSection = group[1].split('-').map(x => Number(x))


    const firstSectionSize = firstSection[1] - firstSection[0]
    const secondSectionSize = secondSection[1] - secondSection[0]

    const firstAssignment = Array.from(Array(firstSectionSize + 1).keys()).map(i => i + firstSection[0])
    const secondAssignment = Array.from(Array(secondSectionSize + 1).keys()).map(i => i + secondSection[0])

    let smaller: Array<number>, bigger: Array<number>;
    if (firstAssignment.length < secondAssignment.length) {
        smaller = firstAssignment;
        bigger = secondAssignment;
    } else {
        smaller = secondAssignment;
        bigger = firstAssignment;
    }
    const hasOverlappedTasks = smaller.some(i => bigger.includes(i))

    console.log("First Assignment:", firstAssignment)
    console.log("Second Assignment:", secondAssignment)
    if (hasOverlappedTasks) {
        overlappingTask += 1
        console.log("+ Overlapped")
    }
    console.log("------------------------------------")
}

console.log(`Number of overlapping taks: ${overlappingTask}`);
