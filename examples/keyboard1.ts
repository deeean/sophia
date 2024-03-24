import * as sophia from '../index';

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

async function main() {
  await sophia.typeText('Hello, World!');
}

main();