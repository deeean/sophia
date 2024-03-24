<div>
  <h1>Sophia</h1>
  <p>
    A library for Windows automation.
  </p>
  
  ![NPM Version](https://img.shields.io/npm/v/@deeean/sophia)
  ![NPM License](https://img.shields.io/npm/l/@deeean/sophia)
  ![Dependents (via libraries.io)](https://img.shields.io/librariesio/dependents/npm/@deeean/sophia)

</div>

## Features
- Keyboard control (partially)
- Mouse control
- Image search
- Window control
- Memory (planned)

## Installation
install with npm:
```bash
npm install @deeean/sophia
```

## Supported Platforms
- Windows x64

## Examples
Typing a text:
```typescript
import * as sophia from '@deeean/sophia';

async function main() {
  await sophia.typeText('Hello, World!');
}

main();
```

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

Draw a circle on Paint:
```typescript
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
```