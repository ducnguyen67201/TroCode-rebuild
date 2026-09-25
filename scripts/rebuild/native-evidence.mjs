import { readFile } from 'node:fs/promises';
import { pathToFileURL } from 'node:url';
import Ajv from 'ajv';

const schema = JSON.parse(
  await readFile(
    new URL(
      '../../tests/acceptance/native-proof-evidence.schema.json',
      import.meta.url,
    ),
    'utf8',
  ),
);
const validate = new Ajv({ strict: false, validateFormats: false }).compile(
  schema,
);
export function assessEvidence(value) {
  if (!validate(value))
    return {
      valid: false,
      accepted: false,
      reasons: ['Evidence does not match the schema.'],
    };
  if (value.version !== 2)
    return {
      valid: true,
      accepted: false,
      reasons: ['Historical v1 evidence cannot satisfy lesson acceptance.'],
    };
  const reasons = [];
  for (const [name, check] of Object.entries(value.checks)) {
    if (check.status !== 'pass') reasons.push(`${name}: ${check.status}`);
  }
  for (const [phase, minimum] of [
    ['localTransition', 20],
    ['stop', 10],
  ]) {
    const samples = value.timings[phase];
    if (samples.length < minimum)
      reasons.push(`${phase}: insufficient samples`);
    else if (
      [...samples].sort((a, b) => a - b)[Math.ceil(samples.length * 0.95) - 1] >
      1000
    )
      reasons.push(`${phase}: p95 exceeds 1000 ms`);
  }
  if (!Number.isFinite(Date.parse(value.observedAt)))
    reasons.push('Invalid observation date.');
  return { valid: true, accepted: reasons.length === 0, reasons };
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(process.argv[1]).href
) {
  try {
    const result = assessEvidence(
      JSON.parse(await readFile(process.argv[2], 'utf8')),
    );
    console.log(JSON.stringify(result, null, 2));
    process.exitCode = result.accepted ? 0 : 1;
  } catch {
    console.error('Cannot read evidence. Supply a local JSON evidence file.');
    process.exitCode = 1;
  }
}
