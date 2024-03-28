import { Keyboard } from "../index";

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

async function main() {
  await Keyboard.typing('Hello, World!');
}

main();