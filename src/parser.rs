use crate::Exercise;
use crate::Muscle;

use std::fs;

pub fn parse_exercises() -> Vec<Exercise> {

    let file_path = "data/exercises.csv";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file.");

    let mut lines = contents.lines();

    // skip first line when we don't need the title
    lines.next();

    // TODOTODO convert to exercise and add to the returned vector
    for line in lines {
        println!("{line}");
    }

    // temp exercise vector
    vec!(Exercise {
        name: String::from("bench press"),
        main_mover: Muscle::Chest
    })
}
