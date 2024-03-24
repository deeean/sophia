import * as sophia from '../index';

async function main() {
  await sophia.keyPress(sophia.Key.LWin);
  await sophia.keyClick(sophia.Key.D);
  await sophia.keyRelease(sophia.Key.LWin);
}


main();