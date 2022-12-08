export default class Elves {
  #elves: Array<number>;

  constructor(max: number) {
    this.#elves = new Array(max).fill(0);
  }

  private add(elf: number) {
    this.#elves.splice(0, 1, elf);
    this.#elves.sort((a, b) => (a > b ? 1 : -1));
  }

  maybeAddElf(elf: number) {
    if (elf <= 0) {
      return;
    }
    if (this.#elves[0] < elf) {
      this.add(elf);
    }
  }

  total() {
    return this.#elves.reduce((prev, curr) => prev + curr, 0);
  }
}
