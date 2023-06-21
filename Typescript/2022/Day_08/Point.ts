
export enum Directions {
    UP = 'U',
    DOWN = 'D',
    LEFT = 'L',
    RIGHT = 'R'
}

export class Point {
    getPoints(): string {
        return JSON.stringify([this.y, this.x]);
    }

    public x: number;
    public y: number;

    private directions = new Map<string, [number, number]>(
        [
            [Directions.UP, [-1, 0]],
            [Directions.DOWN, [1, 0]],
            [Directions.LEFT, [0, -1]],
            [Directions.RIGHT, [0, 1]]
        ]
    )


    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }

    public move(direction: Directions) {
        const directionTuple = this.directions.get(direction);
        if (directionTuple) {
            this.y += directionTuple[0];
            this.x += directionTuple[1]
        }
    }

    public isValid(maxRows: number, maxCols: number, direction: Directions): boolean {

        const res = this.x >= 0 && this.y >= 0 && this.x < maxCols && this.y < maxRows;
        return res;
    }

}
