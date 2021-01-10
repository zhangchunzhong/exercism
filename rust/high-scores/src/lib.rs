#[derive(Debug)]
pub struct HighScores {
    vec: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            vec: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.vec[..]
    }

    pub fn latest(&self) -> Option<u32> {
        match self.vec.len() {
            0 => None,
            n => Some(self.vec[n - 1]),
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        let m = self.vec.iter().max();
        match m {
            None => None,
            Some(i) => Some(*i)
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = self.vec.clone();
        let len = self.vec.len();
        let a = if len >= 3 { 3 } else { len };
        v.sort();
        v.reverse();
        v[0..a].to_vec()
    }
}
