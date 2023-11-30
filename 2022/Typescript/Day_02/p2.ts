import { log } from "console";
import { readFileSync } from "fs";
import * as path from 'path';

const raw_input = readFileSync(path.join(__dirname, "example_input.txt"), 'utf8');

const input = raw_input.split('\n');

enum Plays{
    ROCK = 1,
        PAPER= 2,
        SCISSORS = 3
}

enum Outcome {
    Win =1,
        Draw =  2,
        Defeat = 3
}

const gameMap : { [key: string]:Plays } = {

    "A": Plays.ROCK,
    "B": Plays.PAPER,
    "C": Plays.SCISSORS,
}


const PredictedOutcome : {[key: string]:Outcome}= {
    "X": Outcome.Win, // Opponent Win
    "Y": Outcome.Draw, // Draw!
    "Z": Outcome.Defeat, // Opponent Loses
}

const gameplayOutcomeMap = {
    [ Plays.ROCK ]: {
        [Outcome.Draw] : Plays.ROCK,
        [Outcome.Defeat]: Plays.PAPER,
        [Outcome.Win]: Plays.SCISSORS ,
    },
    [ Plays.PAPER  ]:{
        [Outcome.Draw]: Plays.PAPER,
        [Outcome.Defeat]: Plays.SCISSORS,
        [Outcome.Win]: Plays.ROCK,
    },
    [Plays.SCISSORS]:{
        [Outcome.Draw]: Plays.SCISSORS,
        [Outcome.Defeat]: Plays.ROCK,
        [Outcome.Win]: Plays.PAPER,
    }
}

const gameplayScores = {
    [ Plays.ROCK ]: {
        [ Plays.ROCK ] : Outcome.Draw,
        [ Plays.PAPER ]: Outcome.Defeat,
        [ Plays.SCISSORS ]: Outcome.Win,
    },
    [ Plays.PAPER  ]:{
        [Plays.PAPER ]: Outcome.Draw,
        [ Plays.SCISSORS ]: Outcome.Defeat,
        [ Plays.ROCK ]: Outcome.Win,
    },
    [Plays.SCISSORS]:{
        [Plays.SCISSORS]: Outcome.Draw,
        [Plays.ROCK]: Outcome.Defeat,
        [Plays.PAPER]: Outcome.Win,
    }
}
const WIN_POINTS = 6
const DRAW_POINTS = 3
const LOSE_POINTS = 0

let myScore = 0;
let opponentScore = 0;

for (const game of input){
    if (game == ''){
        continue
    }

    const plays = game.split(' ')
    const opponentPlay = gameMap[plays[0]];
    const gameOutcome = PredictedOutcome[plays[1]]

    console.log(`Plays: ${plays}`)
    console.log(`Game Expected Outcome: ${Outcome[gameOutcome]}`);

    const myPlay = gameplayOutcomeMap[opponentPlay][gameOutcome]
    console.log(`Opponent Play: ${Plays[opponentPlay ]}`)
    console.log(`My Play: ${Plays[myPlay]}`)

    console.log(`Outcome: ${Outcome[gameplayScores[myPlay][opponentPlay]]}`)

    if ( gameplayScores[myPlay][opponentPlay] == Outcome.Win ){
        myScore += WIN_POINTS + myPlay
        opponentScore += LOSE_POINTS + opponentPlay

    }else if(gameplayScores[myPlay][opponentPlay] == Outcome.Draw){
        myScore += DRAW_POINTS + myPlay
        opponentScore += DRAW_POINTS + opponentPlay
        
    }else{
        myScore += LOSE_POINTS + myPlay
        opponentScore += WIN_POINTS + opponentPlay
    }

    console.log("-----------------------------")
}

console.log(`My Score: ${myScore} `)
console.log(`Opponent Score: ${opponentScore} `)
