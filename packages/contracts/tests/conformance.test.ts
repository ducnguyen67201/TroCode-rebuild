import { describe, expect, it } from 'vitest';
import corpus from '../../../tests/fixtures/contracts/corpus.json';
import { parseMessage, parseStatus } from '../src/index';
describe('wire conformance', () => {
  for (const entry of corpus)
    it(entry.name, () => {
      if (entry.valid) expect(() => parseMessage(entry.value)).not.toThrow();
      else
        expect(() => parseMessage(entry.value)).toThrow(
          'Invalid runtime message',
        );
    });
  it('rejects invalid presentation state', () =>
    expect(() => parseStatus({ state: 'healthy' })).toThrow());
});
