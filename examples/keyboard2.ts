import { Keyboard, Key } from "../index";

async function main() {
  await Keyboard.press(Key.LeftWin);
  await Keyboard.click(Key.D);
  await Keyboard.release(Key.LeftWin);
}


main();