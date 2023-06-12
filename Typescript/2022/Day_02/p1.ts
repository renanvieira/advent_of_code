import { readFileSync } from "fs";
import * as path from 'path';

const raw_input = readFileSync(path.join(__dirname, "input.txt"), 'utf8');

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
    "X": Plays.ROCK,
    "Y": Plays.PAPER,
    "Z": Plays.SCISSORS
}

const gameplayScores = {
    [ Plays.ROCK ]: {
        [ Plays.ROCK ] : Outcome.Draw,
        [ Plays.PAPER ]: Outcome.Defeat,
        [ Plays.SCISSORS ]: Outcome.Win,
    },
    [ Plays.PAPER  ]:{
        [ Plays.ROCK ]: Outcome.Win,
        [Plays.PAPER ]: Outcome.Draw,
        [ Plays.SCISSORS ]: Outcome.Defeat,
    },
    [Plays.SCISSORS]:{
        [Plays.ROCK]: Outcome.Defeat,
        [Plays.PAPER]: Outcome.Win,
        [Plays.SCISSORS]: Outcome.Draw
    }
}

const WIN_POINTS = 6
const DRAW_POINTS = 3
const LOSE_POINTS = 0

let myScore = 0;
let opponentScore = 0;

for(let game of input){
    const plays = game.split(' ')
    console.log("The Game:", plays)
    if ( plays.length != 2 || !plays[0] || !plays[1]) {
        continue
    }
    const myPlay : Plays = gameMap[plays[1]] 
    const opponentPlay : Plays = gameMap[plays[0]]


    if ( gameplayScores[myPlay][opponentPlay] == Outcome.Win ){
        myScore += WIN_POINTS + myPlay
        opponentScore += LOSE_POINTS + opponentPlay

        console.log(`You Won: (${WIN_POINTS} + ${myPlay})`, WIN_POINTS + myPlay)
    }else if(gameplayScores[myPlay][opponentPlay] == Outcome.Draw){
        myScore += DRAW_POINTS + myPlay
        opponentScore += DRAW_POINTS + opponentPlay
        
        console.log("You Draw:", DRAW_POINTS + myPlay)
    }else{
        myScore += LOSE_POINTS + myPlay
        opponentScore += WIN_POINTS + opponentPlay
        console.log("You lose:", LOSE_POINTS + myPlay)
    }
}

console.log("My Score:", myScore)
console.log("Opponent Score:", opponentScore)
