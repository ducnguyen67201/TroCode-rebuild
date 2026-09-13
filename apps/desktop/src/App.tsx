import { TeachingPanel } from './features/teaching/TeachingPanel';
import type { DesktopClient } from './platform/desktop-client';
import { RuntimeStatus } from './features/runtime/RuntimeStatus';
export function App({ client }: { client: DesktopClient }) {
  return (
    <main>
      <header>
        <a className="wordmark" href="#">
          tro<span> / foundation</span>
        </a>
        <span className="phase">P1 · Visual teaching foundation</span>
      </header>
      <div className="intro">
        <p className="eyebrow">A solid place to begin</p>
        <h1>
          Make room
          <br />
          for learning.
        </h1>
        <p>
          One desktop. One private runtime.
          <br />A clear foundation for what comes next.
        </p>
      </div>
      {client.preview && <p className="preview">Preview — simulated runtime</p>}
      <RuntimeStatus client={client} />
      <TeachingPanel client={client} />
      <footer>React presentation · Rust supervision · Python runtime</footer>
    </main>
  );
}
