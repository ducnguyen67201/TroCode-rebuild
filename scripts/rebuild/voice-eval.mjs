import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';

const root = resolve(import.meta.dirname, '../..');
const fixtures = JSON.parse(
  await readFile(
    resolve(root, 'tests/fixtures/voice/transcript-overlap.json'),
    'utf8',
  ),
);
if (!Array.isArray(fixtures) || fixtures.length < 3)
  throw new Error('Voice evaluation fixtures are unavailable.');

const spokenMinutes = Number(process.env.TRO_VOICE_EVAL_SPOKEN_MINUTES ?? '1');
if (!Number.isFinite(spokenMinutes) || spokenMinutes <= 0)
  throw new Error('Invalid spoken-minute estimate.');
const billedMinutes = spokenMinutes * 1.25;
const fileCost = billedMinutes * 0.0045;
const liveCost = spokenMinutes * 0.017;

const report = {
  fixtureCount: fixtures.length,
  chunkMilliseconds: 1250,
  overlapMilliseconds: 250,
  spokenMinutes,
  billedMinutes,
  estimatedFileTranscriptionUsd: Number(fileCost.toFixed(6)),
  estimatedLiveTranscriptionUsd: Number(liveCost.toFixed(6)),
  estimatedSavingsPercent: Number(((1 - fileCost / liveCost) * 100).toFixed(1)),
  liveProviderRun: false,
};

if (process.argv.includes('--live')) {
  if (
    process.env.TRO_RUN_LIVE_VOICE_EVAL !== '1' ||
    !process.env.OPENAI_API_KEY
  )
    throw new Error(
      'Live voice evaluation requires explicit TRO_RUN_LIVE_VOICE_EVAL=1 and provider credentials.',
    );
  throw new Error(
    'Run live evaluation through the hosted grant/proxy environment and record the packaged acceptance report; direct provider spend is intentionally unavailable in this script.',
  );
}

console.log(JSON.stringify(report, null, 2));
