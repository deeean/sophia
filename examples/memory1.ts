import * as sophia from '../index';

async function main() {
  const processes = await sophia.getProcesses();
  console.log(processes);
}

main();