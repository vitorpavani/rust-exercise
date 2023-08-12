mod medium {
    pub mod clock;
}

use medium::clock::*;

fn main() {
    println!("{:?}", Clock::new(8, 0).to_string());
    let clock = Clock::new(23, 59).add_minutes(2);
    assert_eq!(clock.to_string(), "00:01");
    println!("{:?}", clock.to_string());
}

// use high_scores::HighScores;
