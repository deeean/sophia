# sophia
> 🚧 Attention! This is a work in progress. The API is not stable and may change at any time.

Sophia is a Node.js library designed for automating desktop tasks, including mouse movements, image searching, and screen captures, inspired by a desire to update and enhance automation capabilities akin to those offered by AutoHotKey.

## Installation
install with npm:
```bash
npm install @deeean/sophia
```

## Usage
Move the mouse cursor:
```typescript
import * as sophia from '@deeean/sophia';

const TAU = Math.PI * 2;

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

async function main() {
  const screenSize = await sophia.getScreenSize();
  const smallest = Math.min(screenSize.x, screenSize.y);
  const radius = smallest / 2;
  const center = { x: screenSize.x / 2, y: screenSize.y / 2 };
  const iteration = 100;

  for (let i = 0; i < iteration; i++) {
    const angle = i * TAU / iteration;
    const x = center.x + radius * Math.cos(angle);
    const y = center.y + radius * Math.sin(angle);
    await sophia.mouseMove(x, y);
    await sleep(10);
  }
}

main();
```

Search an image:
```typescript
import * as sophia from '@deeean/sophia';

async function main() {
  const [
    baboon,
    partsOfBaboon,
  ] = await Promise.all([
    sophia.readImageData('./examples/images/baboon.png'),
    sophia.readImageData('./examples/images/parts_of_baboon.png'),
  ]);

  const position = await sophia.imageSearch(baboon, partsOfBaboon);
  if (position) {
    console.log('Found at', position);
  } else {
    console.log('Not found');
  }
}

main();
```

Search an image on the screen:
```typescript
import * as sophia from '@deeean/sophia';

async function main() {
  const [
    baboon,
  ] = await Promise.all([
    sophia.readImageData('./examples/images/baboon.png'),
  ]);

  const screenSize = await sophia.getScreenSize();
  const screenshot = await sophia.takeScreenshot(0, 0, screenSize.x, screenSize.y);

  const position = await sophia.imageSearch(screenshot, baboon);
  if (position) {
    console.log('Found at', position);
  } else {
    console.log('Not found');
  }
}

main();
```

## Features (WIP)
- [ ] Keyboard
  - [ ] Send Key
  - [ ] Typing
- [x] Mouse
  - [x] Move
  - [x] Click
- [x] Display
  - [x] Get Screen Size
  - [x] Take Screenshot
  - [x] Image Search
  - [x] Multiple Image Search

## Motivation
When I was young, I liked to automate things using AutoHotKey but it's not actively maintained, so I decided to make a new automation framework.