use itertools::Itertools;

#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    top_three: Vec<u32>,
    //latest_index: Option<usize>,
}

impl HighScores {
    pub fn new(scores: & [u32]) -> Self {
        HighScores {
            scores: scores.iter().copied().collect(),
            top_three: scores.iter().copied().sorted().rev().take(3).collect(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.top_three.first().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.top_three.clone()
    }
}
