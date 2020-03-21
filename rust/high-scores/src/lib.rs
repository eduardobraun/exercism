#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores: scores }
    }

    pub fn scores(&self) -> &'a [u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.last() {
            Some(&s) => Some(s),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.scores.iter().max() {
            Some(&s) => Some(s),
            None => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = self.scores.iter().map(|s| *s).collect::<Vec<u32>>();
        v.sort_by(|a, b| b.cmp(a));
        if v.len() <= 3 {
            return v;
        }
        v[0..=2].iter().map(|s| *s).collect::<Vec<_>>()
    }
}
