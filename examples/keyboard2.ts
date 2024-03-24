import * as sophia from '../index';

async function main() {
  await sophia.keyPress(sophia.Key.LeftWin);
  await sophia.keyClick(sophia.Key.D);
  await sophia.keyRelease(sophia.Key.LeftWin);
}


main();