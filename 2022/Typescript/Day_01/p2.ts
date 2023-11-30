
import { readFileSync } from "fs";
import * as path from 'path';

const raw_input = readFileSync(path.join(__dirname, "input.txt"), 'utf8');

const input = raw_input.split('\n');


let kcal_sum = 0
let elf_kcal:Array<number> = []

for(var kcal of input){
    if(kcal != ''){
        kcal_sum += Number(kcal)    
    }else{
        elf_kcal.push(kcal_sum)
        kcal_sum = 0
    }
}


elf_kcal.sort()
const top3_sum = elf_kcal.slice(-3).reduce((i, j)=> i + j, 0)
console.log(top3_sum)
