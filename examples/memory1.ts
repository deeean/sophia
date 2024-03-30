import {getProcesses, openProcess, ProcessAccess} from '../index';

const BASE_ADDRESS = BigInt(0x003264D0);
const OFFSETS = [
  BigInt(0x48),
  BigInt(0x0),
  BigInt(0xF8),
  BigInt(0x18),
  BigInt(0x408),
  BigInt(0x50),
  BigInt(0x7F8),
];

async function main() {
  const processes = await getProcesses();
  const tutorial = processes.find(p => p.name === 'Tutorial-x86_64.exe');

  if (!tutorial) {
    console.log('Tutorial-x86_64.exe not found');
    return;
  }

  const openedProcess = await openProcess(ProcessAccess.AllAccess, tutorial.pid);
  const health = await openedProcess.readMemoryChainUint32(BASE_ADDRESS, OFFSETS);

  console.log('health:', health);
}

main();