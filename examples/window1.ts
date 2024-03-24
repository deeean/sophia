import * as sophia from '../index';

const TAU = Math.PI * 2;

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

async function main() {
  // Should be changed title to your language
  const mspaint = await sophia.findWindowByTitle("제목 없음 - 그림판");
  if (mspaint) {
    await mspaint.setForeground();
    await mspaint.setMaximize();
  }

  let x = 400;
  let y = 600;

  await sophia.mouseMove(x, y);
  await sophia.mousePress(sophia.MouseButton.Left);

  const iterations = 100;

  for (let i = 0; i < iterations; i++) {
    x += Math.sin(i / iterations * TAU) * 10;
    y += Math.cos(i / iterations * TAU) * 10;
    await sophia.mouseMove(x, y);

    if (i === 0) {
      await sophia.mousePress(sophia.MouseButton.Left);
    }

    await sleep(10);
  }

  await sophia.mouseRelease(sophia.MouseButton.Left);
}

main();