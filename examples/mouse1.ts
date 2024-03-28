import { Mouse, getScreenSize } from "../index";

const TAU = Math.PI * 2;

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

async function main() {
  const screenSize = await getScreenSize();
  const smallest = Math.min(screenSize.x, screenSize.y);
  const radius = smallest / 2;
  const center = { x: screenSize.x / 2, y: screenSize.y / 2 };
  const iteration = 200;

  for (let i = 0; i < iteration; i++) {
    const angle = i * TAU / iteration;
    const x = center.x + radius * Math.cos(angle);
    const y = center.y + radius * Math.sin(angle);
    await Mouse.move(x, y);
    await sleep(10);
  }
}

main();