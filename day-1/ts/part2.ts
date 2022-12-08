import { getData } from "./client";
import Elves from "./elves";

async function main() {
  const session = process.argv[2];
  const data = await getData(session);

  const elves = new Elves(3);
  let curr = 0;
  for (const line of data) {
    if (line.trim() === "") {
      elves.maybeAddElf(curr);
      curr = 0;
      continue;
    }
    curr += parseInt(line, 10);
  }
  console.log(elves.total());
}

main();
