use crate::Exercise;
use crate::Muscle;

use std::ascii::AsciiExt;
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

    for line in lines {
//        let line: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        let parts: Vec<&str> = line.split(",").collect();

        exercises.push(
            Exercise {
                name: parts[0].to_string(),
                main_mover: parts[1].parse().unwrap(),
                movement_pattern: parts[2].parse().unwrap()
            }
        )
    }

    println!("Exercises: {:?}", exercises);
    return exercises;
}

pub fn parse_bodyweights() -> Vec<(String, f64)> {

    let file_path = "data/bodyweight.csv";
    let mut bodyweights: Vec<(String, f64)> = vec!();

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file.");

    let mut lines = contents.lines();

    // skip first line when we don't need the title
    lines.next();

    for line in lines {
        let parts: Vec<&str> = line.split(",").collect();

        bodyweights.push(
            (parts[0].to_string(), parts[1].parse().expect("Failed to parse f32"))
        );
    }

    println!("Bodyweights: {:?}", bodyweights);
    bodyweights
}
