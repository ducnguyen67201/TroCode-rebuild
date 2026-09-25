import { spawn } from 'node:child_process';
const owned = new Set();
const stopping = new WeakSet();
let interruption;
export function start(command, args, options = {}) {
  if (interruption) throw interruption;
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
      if (interruption) reject(interruption);
      else if (stopping.has(child) || code === 0) resolve();
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
      if (error.code === 'ESRCH') return;
      if (error.code === 'EPERM') {
        // The process group may already be changing after a forwarded Ctrl-C.
        // Fall back to the exact child instead of aborting cleanup for siblings.
        try {
          child.kill('SIGTERM');
        } catch (fallbackError) {
          if (fallbackError.code !== 'ESRCH') throw fallbackError;
        }
        return;
      }
      throw error;
    }
  }
}
export function stopOwned() {
  for (const child of owned) stopChild(child);
}
export function interruptOwned(signal) {
  interruption ??= new Error(`Workflow interrupted by ${signal}`);
  stopOwned();
}
