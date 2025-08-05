#[derive(Debug, Clone)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.clone().into_iter().last()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.clone().into_iter().max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.clone();
        scores.sort_by(|a, b| b.cmp(a));

        let length = if scores.len() > 3 {
            3
        } else {
            scores.len()
        };

        scores[..length].to_vec()
    }
}
