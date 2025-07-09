import fs from "fs";

const SKY_SIZE = 1000;
const FRAME_SIZE = 500;
const FRAME_START = (SKY_SIZE - FRAME_SIZE) / 2;
const FRAME_END = FRAME_START + FRAME_SIZE;
const NUM_PICTURES = 1000;
const SECONDS_PER_PICTURE = 3600;

function parseInput(path: string): { x: number; y: number }[] {
  const fileContents = fs.readFileSync(path, "utf8");
  const result = fileContents.split("\n");
  const splitted = result.map((line) => {
    const [x, y] = line.split(",");
    return { x: parseInt(x!), y: parseInt(y!) };
  });

  return splitted;
}

function main() {
  let total = 0;

  let birdSpeeds = parseInput("input.txt");

  for (let h = 0; h < NUM_PICTURES; h++) {
    let count = 0;
    const t = h * SECONDS_PER_PICTURE;

    birdSpeeds.forEach((bird) => {
      const x = wrapModulo(bird.x * t, SKY_SIZE);
      const y = wrapModulo(bird.y * t, SKY_SIZE);

      if (
        x >= FRAME_START &&
        x <= FRAME_END &&
        y >= FRAME_START &&
        y <= FRAME_END
      ) {
        count++;
      }
    });

    total += count;
  }

  console.log(total);
}

function wrapModulo(n: number, m: number): number {
  return ((n % m) + m) % m;
}

main();
