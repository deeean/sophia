import { Window, Mouse, MouseButton } from '../index';

const TAU = Math.PI * 2;

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

async function main() {
  const mspaint = await Window.findWindowByTitle("Untitled - Paint");
  if (mspaint) {
    await mspaint.foreground();
    await mspaint.maximize();
  }

  let x = 400;
  let y = 600;

  await Mouse.move(x, y);
  await Mouse.press(MouseButton.Left);

  const iterations = 100;

  for (let i = 0; i < iterations; i++) {
    x += Math.sin(i / iterations * TAU) * 10;
    y += Math.cos(i / iterations * TAU) * 10;
    await Mouse.move(x, y);

    if (i === 0) {
      await Mouse.press(MouseButton.Left);
    }

    await sleep(10);
  }

  await Mouse.release(MouseButton.Left);
}

main();