import assert from "assert";
import { getData } from "./client";

const RockPaperScissors = {
  A: 1,
  B: 2,
  C: 3,
};

type Input = keyof typeof RockPaperScissors;

type Result = "X" | "Y" | "Z";

const LOSE_AGAINST: Record<Input, Input> = {
  A: "B",
  B: "C",
  C: "A",
};

const WINS_TO: Record<Input, Input> = {
  A: "C",
  B: "A",
  C: "B",
};

function getPoints(they: Input, result: Result) {
  let outcomePoints;
  let myShapePoints;
  if (result === "X") {
    outcomePoints = 0;
    myShapePoints = RockPaperScissors[WINS_TO[they]];
  } else if (result === "Y") {
    outcomePoints = 3;
    myShapePoints = RockPaperScissors[they];
  } else {
    outcomePoints = 6;
    myShapePoints = RockPaperScissors[LOSE_AGAINST[they]];
  }

  return outcomePoints + myShapePoints;
}

function assertIsValidInput(input: string): asserts input is Input {
  assert(input === "A" || input === "B" || input === "C", input);
}

function assertIsValidResult(result: string): asserts result is Result {
  assert(result === "X" || result === "Y" || result === "Z", result);
}

async function main() {
  const session = process.argv[2];
  const data = await getData(session);

  let points = 0;
  for (const l of data) {
    const line = l.trim();
    if (!line) {
      continue;
    }
    const [they, me] = line.split(" ");
    assertIsValidInput(they);
    assertIsValidResult(me);
    points += getPoints(they, me);
  }
  console.log(points);
}

main();
