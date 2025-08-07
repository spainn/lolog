use crate::Exercise;
use crate::Muscle;

use std::fs;

// need a string to muscle macro to make sure that each muscle string is converted
// into the correct enum variant

pub fn parse_exercises() -> Vec<Exercise> {

    let file_path = "data/exercises.csv";
    let mut exercises: Vec<Exercise> = vec!();

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file.");

    let mut lines = contents.lines();

    // skip first line when we don't need the title
    lines.next();

    // TODOTODO convert to exercise and add to the returned vector
    for line in lines {
        println!("{line}");
        let no_whitespace_line: String = line.chars().filter(|c| !c.is_whitespace()).collect();

        let parts: Vec<&str> = no_whitespace_line.split(",").collect();

        for part in &parts {
            println!("{part}");
        }
        exercises.push(
            Exercise {
                name: parts[0].to_string(),
                main_mover: parts[1].parse().unwrap(),
                movement_pattern: parts[2].parse().unwrap()
            }
        )

    }


    return exercises;
}
