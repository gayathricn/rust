use high_scores::HighScores;
fn main(){
    let scores = vec![30, 10, 50, 20, 40];
    let high_scores = HighScores::new(&scores);
    println!("All scores: {:?} ", high_scores.scores());
    println!("Latest score: {:?} ", high_scores.latest());
    println!("High score: {:?} ", high_scores.personal_best());
    println!("Top three scores: {:?} ", high_scores.personal_top_three());

}