import axios from "axios";

const URL = "https://adventofcode.com/2022/day/2/input";

async function getData(session: string) {
  const response = await axios.get(URL, {
    headers: {
      cookie: `session=${session}`,
    },
  });

  const { data } = response;
  const lines: Array<string> = data.split("\n");
  return lines;
}

export { getData };
