import { registerHotkey, Modifiers, Key, mouseMove } from '../index';

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

async function main() {
  registerHotkey([Modifiers.Control], Key.A, async () => {
    await mouseMove(100, 100);
    await sleep(100);
    await mouseMove(200, 100);
    await sleep(100);
    await mouseMove(200, 200);
    await sleep(100);
    await mouseMove(100, 200);
    await sleep(100);
    await mouseMove(100, 100);
  });
}


main();