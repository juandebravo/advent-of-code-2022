import assert from "assert";
import { getData } from "./client";

const RockPaperScissors = {
  A: 1,
  B: 2,
  C: 3,
  X: 1,
  Y: 2,
  Z: 3,
};

type Input = keyof typeof RockPaperScissors;

const THEY_BEAT_ME: Partial<Record<Input, Input>> = {
  A: "Z",
  B: "X",
  C: "Y",
};

function getPoints(they: Input, me: Input) {
  let outcomePoints = 3;
  if (RockPaperScissors[they] !== RockPaperScissors[me]) {
    outcomePoints = THEY_BEAT_ME[they] === me ? 0 : 6;
  }

  let myShapePoints = RockPaperScissors[me];
  return outcomePoints + myShapePoints;
}

function assertIsValidInput(input: string): asserts input is Input {
  assert(
    input === "A" ||
      input === "B" ||
      input === "C" ||
      input === "X" ||
      input === "Y" ||
      input === "Z",
    input
  );
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
    assertIsValidInput(me);
    points += getPoints(they, me);
  }
  console.log(points);
}

main();
