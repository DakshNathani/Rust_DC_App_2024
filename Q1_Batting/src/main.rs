use std::fmt;

struct Game { // creating a data structure
    overs: Vec<Vec<i32>>,
    total_score: i32,     
    extra_overs: i32,     
}

impl Game { // implementing the data structure create
    fn new() -> Self { // self is used for the same data structure
        Game { //creates new empty vector and initializes everything to 0
            overs: Vec::new(),
            total_score: 0,
            extra_overs: 0,
        }
    }

    fn bat(&mut self, runs: Vec<i32>) { // to add runs as list
        if runs.len() != 6 {
            panic!("An over must have exactly 6 balls.");
        }
        self.overs.push(runs); // appends the list
    }

    fn score(&self) -> i32 { // calculates the total score
        let mut total = 0;
        for over in &self.overs {
            for &run in over {
                total += run;
            }
        }
        total // will return the total
    }
}

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
