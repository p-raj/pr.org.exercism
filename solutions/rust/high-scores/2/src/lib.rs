#[derive(Debug)]
pub struct HighScores{
    scores: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.iter().copied().collect()
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores[..]
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.len() {
            0 => None,
            _ => Some(self.scores[self.scores.len() - 1])
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.scores.len() {
            0 => None,
            _ => {
                let mut scores = self.scores[..].to_vec();
                scores.sort();
                scores.reverse();
                Some(scores[0])
            }
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores[..].to_vec();
        scores.sort();
        scores.reverse();
        match self.scores.len() {
            0 => vec![],
            1 => scores[0..1].to_vec(),
            2 => scores[0..2].to_vec(),
            _ => scores[0..3].to_vec()
        }
    }
}
