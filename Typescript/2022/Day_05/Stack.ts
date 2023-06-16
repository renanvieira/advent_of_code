export class Stack<T>{

    private storage: Array<T> = [];

    public push(items: Array<T>): void {
        this.storage = items.concat(this.storage);
    }

    public pop(quantity: number=1): Array<T> {
        return this.storage.splice(0, quantity);
    }

    public size(): number {
        return this.storage.length;
    }

    public peek(pos: number): T | undefined {
        return this.storage.at(pos);
    }


}

