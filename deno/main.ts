import { copy } from "https://deno.land/std@0.167.0/streams/conversion.ts";

const text: string = await Deno.readTextFile("../data/03-input");
const ruck = text.split("\n");

let firstSolution = partOne(ruck);
let secondSolution = partTwo(ruck);

console.log(`=== First solution: ${firstSolution} ===`);
console.log(`=== Second solution: ${secondSolution} ===`);

function partOne(input: string[]): number {
  let commonsItems: string[] = [];
  for (const w of input) {
    const common = findCommonElement(w);
    if (common != "") commonsItems.push(common);
  }
  console.table(commonsItems);

  let prioritySum = sumPriority(commonsItems);

  console.log(prioritySum);
  return prioritySum;
}
function sumPriority(commonsItems: string[]) {
  const priority = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

  return commonsItems.reduce((prev, curr) => {
    console.log(
      `prev: ${prev} | curr: ${curr} | idx: ${priority.indexOf(curr)}`
    );
    return (prev += priority.indexOf(curr) + 1);
  }, 0);
}

function findCommonElement(w: string) {
  const first = w.substring(0, w.length / 2);
  const second = w.substring(w.length / 2);
  console.debug(`First: ${first}`);
  console.debug(`Second: ${second}`);

  let common = "";

  for (const i of first) {
    if (second.indexOf(i) >= 0) common = i;
  }

  console.debug(`Found common element: ${common}`);
  return common;
}
function partTwo(ruck: string[]): number {
  let badges: string[] = [];
  for (let i = 0; i < ruck.length; i = i + 3) {
    const common = findCommonElementInThree(ruck[i], ruck[i + 1], ruck[i + 2]);
    badges.push(common);
  }
  return sumPriority(badges);
}

function findCommonElementInThree(s1: string, s2: string, s3: string): string {
  let longest = s1;

  if (s2.length > longest.length) longest = s2;
  if (s3.length > longest.length) longest = s3;

  for (let c of longest) {
    if (s1.indexOf(c) >= 0 && s2.indexOf(c) >= 0 && s3.indexOf(c) >= 0)
      return c;
  }

  throw new Error("Have not found any common element.");
}
