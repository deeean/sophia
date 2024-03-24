import * as sophia from '../index';

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