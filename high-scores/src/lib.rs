//use std::backtrace;

#[derive(Debug)]
pub struct HighScores{
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        //todo!("Construct a HighScores struct, given the scores: {scores:?}")
        HighScores{
            scores:scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        //todo!("Return all the scores as a slice"
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        //todo!("Return the latest (last) score")
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        //todo!("Return the highest score ")
        self.scores.iter().copied().max()        
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        //todo!("Return 3 highest scores ")
        let mut top_three: Vec<u32> = self.scores.iter().copied().collect();
        top_three.sort_unstable_by(|a,b| b.cmp(a));
        top_three.truncate(3);
        top_three
    }
}
