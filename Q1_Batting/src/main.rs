use std::fmt;

struct Game {
    overs: Vec<Vec<i32>>, // Store runs for each over (each over is a Vec<i32>)
    total_score: i32,     // Store the final score
    extra_overs: i32,     // Track the number of extra overs needed
}

impl Game {
    // Constructor to create a new game with empty overs
    fn new() -> Self {
        Game {
            overs: Vec::new(),
            total_score: 0,
            extra_overs: 0,
        }
    }

    // Bat method: add runs for each ball in the current over
    fn bat(&mut self, runs: Vec<i32>) {
        if runs.len() != 6 {
            panic!("An over must have exactly 6 balls.");
        }
        self.overs.push(runs);
    }

    // Calculate the total score based on the runs in each over
    fn score(&self) -> i32 {
        let mut total = 0;
        for over in &self.overs {
            for &run in over {
                total += run;
            }
        }
        total
    }
}

// Implement fmt::Display to print the scorecard
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, over) in self.overs.iter().enumerate() {
            write!(f, "Over {}: (", i + 1)?;
            for (j, &run) in over.iter().enumerate() {
                if run >= 4 {
                    write!(f, "*{}", run)?; // Prefix boundaries with '*'
                } else {
                    write!(f, "{}", run)?;
                }
                if j < 5 {
                    write!(f, ", ")?; // Comma between balls
                }
            }
            writeln!(f, ")")?;
        }
        writeln!(f, "Final Score: {}", self.score())
    }
}

fn main() {
    let mut game = Game::new();

    // Example overs
    game.bat(vec![6, 6, 4, 4, 6, 4]); // Over 1
    game.bat(vec![1, 1, 2, 3, 0, 0]); // Over 2

    // Print the scorecard
    println!("{}", game);
}
