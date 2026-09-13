import { spawn } from 'node:child_process';
const owned = new Set();
const stopping = new WeakSet();
export function start(command, args, options = {}) {
  const child = spawn(command, args, {
    stdio: 'inherit',
    shell: false,
    detached: process.platform !== 'win32',
    ...options,
  });
  owned.add(child);
  const done = new Promise((resolve, reject) => {
    child.once('error', (error) => {
      owned.delete(child);
      reject(error);
    });
    child.once('exit', (code, signal) => {
      owned.delete(child);
      if (
        stopping.has(child) ||
        code === 0 ||
        signal === 'SIGTERM' ||
        signal === 'SIGINT'
      )
        resolve();
      else reject(new Error(`${command} exited with ${code ?? signal}`));
    });
  });
  return { child, done };
}
export async function run(command, args, options) {
  await start(command, args, options).done;
}
export function stopChild(child) {
  if (!owned.has(child) || !child.pid) return;
  stopping.add(child);
  if (process.platform === 'win32') {
    // Restrict termination to the process tree created by this command.
    spawn('taskkill', ['/pid', String(child.pid), '/T', '/F'], {
      stdio: 'ignore',
      shell: false,
    }).on('error', () => child.kill());
  } else {
    try {
      process.kill(-child.pid, 'SIGTERM');
    } catch (error) {
      if (error.code !== 'ESRCH') throw error;
    }
  }
}
export function stopOwned() {
  for (const child of owned) stopChild(child);
}
