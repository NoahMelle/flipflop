import fs from "fs";

function parseInput(path: string) {
  const fileContents = fs.readFileSync(path, "utf8");
  const result = fileContents.split("\n");

  return result;
}

function main() {
  const inputPath = "input.txt";
  const lines = parseInput(inputPath);

  const SKY_SIZE = 1000;
  const FRAME_SIZE = 500;
  const SECONDS = 100;

  let birdSpeeds = lines.map((line) => {
    const [x, y] = line.split(",");
    return {
      x: parseInt(x!),
      y: parseInt(y!),
    };
  });

  let birdPositions: { x: number; y: number }[] = Array.from(
    { length: birdSpeeds.length },
    () => ({ x: 0, y: 0 })
  );

  for (let s = 0; s < SECONDS; s++) {
    birdPositions = birdPositions.map((bird, index) => {
      const newX = (bird.x + birdSpeeds[index]!.x + SKY_SIZE) % SKY_SIZE;
      const newY = (bird.y + birdSpeeds[index]!.y + SKY_SIZE) % SKY_SIZE;
      return { x: newX, y: newY };
    });
  }

  const birdsInFrame = birdPositions.filter((bird) => {
    const frameStart = (SKY_SIZE - FRAME_SIZE) / 2;
    const frameEnd = frameStart + FRAME_SIZE;
    return (
      bird.x >= frameStart &&
      bird.x < frameEnd &&
      bird.y >= frameStart &&
      bird.y < frameEnd
    );
  });

  console.log(birdsInFrame.length);
  // console.log(birdPositions);
}

main();
