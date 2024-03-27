<div align="center">
  <h1>Sophia</h1>
  <p>
    <b>Sophia</b> is a library for automating Windows applications.
  </p>
  
  ![NPM Version](https://img.shields.io/npm/v/@deeean/sophia)
  ![NPM License](https://img.shields.io/npm/l/@deeean/sophia)
  ![Dependents (via libraries.io)](https://img.shields.io/librariesio/dependents/npm/@deeean/sophia)

</div>

## Features
- Keyboard
  - Send keys
  - Hotkey
- Mouse
  - Move
  - Click
- Screen
  - Image search
  - Multiple image search
- Window
  - Find by title
  - Foreground
  - Maximize
  - Minimize
- Memory
  - Get processes
  - Read memory (planned)
  - Write memory (planned)

## Installation
install with npm:
```bash
npm install @deeean/sophia
```

## Supported Platforms
Only support Windows x64 for now.

## Examples
### [Aim Test](https://www.arealme.com/aim-test/en/)
<img src="https://media.deeean.com/sophia_aimtest.gif" />

Typing a text:
```typescript
await sophia.typeText('Hello, World!');
```

Hotkey Ctrl + A:
```typescript
sophia.registerHotkey([sophia.Modifiers.Control], sophia.Key.A, async () => {
  await sophia.mouseMove(100, 100);
  await sophia.sleep(100);
  await sophia.mouseMove(200, 100);
  await sophia.sleep(100);
  await sophia.mouseMove(200, 200);
  await sophia.sleep(100);
  await sophia.mouseMove(100, 200);
  await sophia.sleep(100);
  await sophia.mouseMove(100, 100);
});
```

Send Win + D:
```typescript
await sophia.keyPress(sophia.Key.LeftWin);
await sophia.keyClick(sophia.Key.D);
await sophia.keyRelease(sophia.Key.LeftWin);
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

## Related projects
- [AutoHotkey](https://github.com/AutoHotkey/AutoHotkey)
- [PyAutoGUI](https://github.com/asweigart/pyautogui)
- [RobotJS](https://github.com/octalmage/robotjs)

## License
Sophia is licensed under the MIT License. Feel free to use it in your projects, adhering to the license terms.