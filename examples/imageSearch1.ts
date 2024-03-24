import * as sophia from '../index';

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