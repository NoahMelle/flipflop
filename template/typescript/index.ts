import fs from "fs";

function parseInput(path: string) {
  const fileContents = fs.readFileSync(path, "utf8");
  const result = fileContents.split("\n");

  return result;
}

function main() {
  const inputPath = "input.txt";
  const lines = parseInput(inputPath);

  lines.forEach((line) => {
    console.log(line);
  });
}

main();
