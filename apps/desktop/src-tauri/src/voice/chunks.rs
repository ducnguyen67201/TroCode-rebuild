use hound::{SampleFormat, WavSpec, WavWriter};
use std::io::Cursor;

pub const CHUNK_MS: u64 = 1_250;
pub const OVERLAP_MS: u64 = 250;
pub const MAX_UTTERANCE_MS: u64 = 30_000;

#[derive(Clone, Debug)]
pub struct WavChunk {
    pub sequence: u32,
    pub duration_ms: u64,
    pub final_chunk: bool,
    pub bytes: Vec<u8>,
}

pub struct ChunkAssembler {
    sample_rate: u32,
    samples: Vec<i16>,
    next_start: usize,
    sequence: u32,
}

impl ChunkAssembler {
    pub fn new(sample_rate: u32) -> Result<Self, &'static str> {
        if !(8_000..=192_000).contains(&sample_rate) {
            return Err("Unsupported microphone sample rate.");
        }
        Ok(Self {
            sample_rate,
            samples: Vec::new(),
            next_start: 0,
            sequence: 0,
        })
    }

    pub fn push(&mut self, incoming: &[i16]) -> Result<Vec<WavChunk>, &'static str> {
        let max_samples = self.samples_for_ms(MAX_UTTERANCE_MS);
        if self.samples.len().saturating_add(incoming.len()) > max_samples {
            return Err("Voice instruction is too long.");
        }
        self.samples.extend_from_slice(incoming);
        let chunk_len = self.samples_for_ms(CHUNK_MS);
        let step = self.samples_for_ms(CHUNK_MS - OVERLAP_MS);
        let mut output = Vec::new();
        while self.samples.len().saturating_sub(self.next_start) >= chunk_len {
            let end = self.next_start + chunk_len;
            output.push(self.encode(self.next_start, end, false)?);
            self.next_start += step;
        }
        Ok(output)
    }

    pub fn finish(&mut self) -> Result<Option<WavChunk>, &'static str> {
        if self.samples.len() <= self.next_start {
            return Ok(None);
        }
        let end = self.samples.len();
        Ok(Some(self.encode(self.next_start, end, true)?))
    }

    pub fn has_speech(&self) -> bool {
        if self.samples.len() < self.samples_for_ms(120) {
            return false;
        }
        let energy = self
            .samples
            .iter()
            .map(|sample| i64::from(*sample).abs())
            .sum::<i64>();
        energy / self.samples.len() as i64 >= 96
    }

    pub fn clear(&mut self) {
        use zeroize::Zeroize;
        self.samples.zeroize();
        self.samples.clear();
    }

    fn samples_for_ms(&self, milliseconds: u64) -> usize {
        (u64::from(self.sample_rate) * milliseconds / 1_000) as usize
    }

    fn encode(
        &mut self,
        start: usize,
        end: usize,
        final_chunk: bool,
    ) -> Result<WavChunk, &'static str> {
        let mut cursor = Cursor::new(Vec::new());
        {
            let mut writer = WavWriter::new(
                &mut cursor,
                WavSpec {
                    channels: 1,
                    sample_rate: self.sample_rate,
                    bits_per_sample: 16,
                    sample_format: SampleFormat::Int,
                },
            )
            .map_err(|_| "Unable to encode microphone audio.")?;
            for sample in &self.samples[start..end] {
                writer
                    .write_sample(*sample)
                    .map_err(|_| "Unable to encode microphone audio.")?;
            }
            writer
                .finalize()
                .map_err(|_| "Unable to encode microphone audio.")?;
        }
        let duration_ms = ((end - start) as u64 * 1_000) / u64::from(self.sample_rate);
        let chunk = WavChunk {
            sequence: self.sequence,
            duration_ms,
            final_chunk,
            bytes: cursor.into_inner(),
        };
        self.sequence += 1;
        Ok(chunk)
    }
}

impl Drop for ChunkAssembler {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_completed_wav_with_overlap_and_final_tail() {
        let mut assembler = ChunkAssembler::new(48_000).unwrap();
        let chunks = assembler.push(&vec![500; 48_000 * 1420 / 1000]).unwrap();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].duration_ms, 1_250);
        assert!(!chunks[0].final_chunk);
        assert_eq!(&chunks[0].bytes[0..4], b"RIFF");
        let tail = assembler.finish().unwrap().unwrap();
        assert_eq!(tail.duration_ms, 420);
        assert!(tail.final_chunk);
        assert!(assembler.has_speech());
    }

    #[test]
    fn rejects_silence_and_overlong_capture() {
        let mut assembler = ChunkAssembler::new(16_000).unwrap();
        assembler.push(&vec![0; 16_000]).unwrap();
        assert!(!assembler.has_speech());
        assert!(assembler.push(&vec![0; 16_000 * 30]).is_err());
    }
}
