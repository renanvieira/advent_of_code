import { Stack } from "./Stack";


export class CargoBay {

    protected stacks: Array<Stack<string>> = [];

    constructor(initial_map: Array<Stack<string>>) {
        this.stacks = initial_map;
    }


    public moveContainer(source: number, destination: number, quantity: number = 1) {

        const src = this.stacks[source];
        const dst = this.stacks[destination];

        Array.from({ length: quantity }, x => {
            const item = src.pop();

            dst.push(item);
        });

    }

    public getTopItemsFromStacks(): Array<string | undefined> {
        return this.stacks.map(x => x.peek(0))
    }
}



export class CargoBayPart2 extends CargoBay {


    public moveContainer(source: number, destination: number, quantity?: number): void {

        const src = this.stacks[source];
        const dst = this.stacks[destination];

        const item = src.pop(quantity);

        if (item) {
            dst.push(item);
        }
    }
}
