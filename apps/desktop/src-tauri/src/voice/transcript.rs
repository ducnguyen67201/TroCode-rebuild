use std::collections::BTreeMap;

pub const MAX_IN_FLIGHT: usize = 2;
const MAX_OVERLAP_TOKENS: usize = 24;

#[derive(Default)]
pub struct TranscriptAssembler {
    next_sequence: u32,
    pending: BTreeMap<u32, String>,
    stable: String,
    frozen: bool,
}

impl TranscriptAssembler {
    pub fn insert(&mut self, sequence: u32, text: String) -> Result<&str, &'static str> {
        if self.frozen || text.len() > 2_000 || sequence < self.next_sequence {
            return Err("Invalid transcript chunk.");
        }
        if self.pending.insert(sequence, text).is_some() {
            return Err("Duplicate transcript chunk.");
        }
        while let Some(text) = self.pending.remove(&self.next_sequence) {
            self.stable = merge_overlap(&self.stable, &text);
            self.next_sequence += 1;
        }
        Ok(&self.stable)
    }

    pub fn prompt_tail(&self) -> String {
        let mut chars: Vec<char> = self.stable.chars().rev().take(500).collect();
        chars.reverse();
        chars.into_iter().collect()
    }

    pub fn finish(&mut self, expected_chunks: u32) -> Result<String, &'static str> {
        if self.frozen || !self.pending.is_empty() || self.next_sequence != expected_chunks {
            return Err("Transcription is incomplete.");
        }
        let result = self.stable.trim().to_owned();
        if result.is_empty() {
            return Err("No speech was detected.");
        }
        self.frozen = true;
        Ok(result)
    }
}

pub fn merge_overlap(stable: &str, incoming: &str) -> String {
    let left: Vec<&str> = stable.split_whitespace().collect();
    let right: Vec<&str> = incoming.split_whitespace().collect();
    let max = left.len().min(right.len()).min(MAX_OVERLAP_TOKENS);
    let overlap = (1..=max)
        .rev()
        .find(|size| {
            left[left.len() - size..]
                .iter()
                .zip(&right[..*size])
                .all(|(a, b)| normalize(a) == normalize(b))
        })
        .unwrap_or(0);
    if overlap > 0 {
        return left[..left.len() - overlap]
            .iter()
            .chain(right.iter())
            .copied()
            .collect::<Vec<_>>()
            .join(" ");
    }
    let mut merged = stable.trim_end().to_owned();
    let remainder = right.join(" ");
    if !merged.is_empty() && !remainder.is_empty() && needs_space(&merged, &remainder) {
        merged.push(' ');
    }
    merged.push_str(&remainder);
    merged
}

fn normalize(value: &str) -> String {
    value
        .trim_matches(|character: char| character.is_ascii_punctuation())
        .to_lowercase()
}

fn needs_space(left: &str, right: &str) -> bool {
    !right.starts_with(|character: char| ",.!?:;)".contains(character))
        && !left.ends_with(|character: char| "(“‘".contains(character))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_out_of_order_english_overlap() {
        let mut transcript = TranscriptAssembler::default();
        assert_eq!(
            transcript.insert(1, "the settings page".into()).unwrap(),
            ""
        );
        assert_eq!(
            transcript.insert(0, "open the settings".into()).unwrap(),
            "open the settings page"
        );
        assert_eq!(transcript.finish(2).unwrap(), "open the settings page");
    }

    #[test]
    fn preserves_vietnamese_and_punctuation() {
        assert_eq!(
            merge_overlap("mở phần cài đặt", "cài đặt, rồi cuộn xuống"),
            "mở phần cài đặt, rồi cuộn xuống"
        );
    }
}
