import { Keyboard, Mouse, Modifiers, Key } from '../index';

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

async function main() {
  Keyboard.registerHotkey([Modifiers.Control], Key.A, async () => {
    await Mouse.move(100, 100);
    await sleep(100);
    await Mouse.move(200, 100);
    await sleep(100);
    await Mouse.move(200, 200);
    await sleep(100);
    await Mouse.move(100, 200);
    await sleep(100);
    await Mouse.move(100, 100);
  });
}


main();