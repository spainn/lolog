use std::fs;

use crate::exercise::*;

pub fn parse_movements() -> Vec<Movement> {
    const MOVEMENTS_FILE: &str = "movements.txt";

    println!("In file {MOVEMENTS_FILE}");

    let contents =
        fs::read_to_string(MOVEMENTS_FILE).expect("should have been able ot read this file");
    let mut workouts = Vec::new();
    let mut movements = Vec::new();
    let mut current_section = "";

    for line in contents.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        match line {
            "[workouts]" => current_section = "workouts",
            "[movements]" => current_section = "movements",
            _ => match current_section {
                "workouts" => workouts.push(line.to_string()),
                "movements" => {
                    movements.push(parse_movement_line(line));
                }
                _ => {}
            },
        }
    }

    movements
}

fn parse_movement_line(line: &str) -> Movement {
    let args: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

    Movement {
        name: args[0].to_string(),
        alias: args[1].to_string(),
        main_mover: args[2].to_string(),
        default_to_failure: match args[3] {
            "true" => true,
            "t" => true,
            "True" => true,
            "T" => true,
            "false" => false,
            "f" => false,
            "False" => false,
            "F" => false,
            _ => true,
        },
    }
}

// parses a complete workout from the input.txt file
// if only the name of a workout is given that is loaded from
// movements.txt, that workout will be loaded and the actual
// reps / weight will be asked, and modifications can be made
//
// if not it loads in the text and parses it from the given
// format, then asks which exercise was meant by the given
// titles.  each workout shoudl start with a date
pub fn parse_complete_workout() {
    const INPUT_FILE: &str = "input.txt";

    println!("In file {INPUT_FILE}");

    let contents =
        fs::read_to_string(INPUT_FILE).expect("should have been able ot read this file");

    let mut exercises: Vec<Exercise> = Vec::new();
    let mut position = 1; // the set # in the entire workout
    let mut lines = contents.lines();
    let date = lines.next();

    for line in lines {
        // each line needs parsed into an Exercise
        let trimmed = line.trim();
        let split: Vec<&str> = trimmed.split(' ').filter(|c| !c.is_empty()).collect();

//        split[0] is set count
//        split[1] needs split again to create rep counts
//        split[2] needs split again to create weights
//        split[3] onwards need joined with spaces inbetween to get the name

        let set_count: u32 = split[0].parse().expect("expected u32 value");
        // parse the number of reps completed
        let mut rep_counts: Vec<u32> = split[1]
            .split(",")
            .map(|r| r.parse().expect("expected u32 value"))
            .collect();

        while rep_counts.len() < set_count.try_into().unwrap() {
            rep_counts.push(*rep_counts.get(rep_counts.len()-1).expect("expected u32"));
        }

        // parse weights used
        let mut weights: Vec<u32> = split[2]
            .split(",")
            .map(|r| r.parse().expect("expected u32 value"))
            .collect();

        while weights.len() < set_count.try_into().unwrap() {
            weights.push(*weights.get(weights.len()-1).expect("expected u32 here"))
        }

        // process name
        let slice = &split[3..];
        let name = slice.join(" ");
        let mut sets: Vec<Set> = Vec::new();

        for (reps, weight) in rep_counts.iter().zip(weights.iter()) {
            let set = Set { position: position, reps: *reps, weight: *weight };
            sets.push(set);
            position += 1;
        }

        // find the movement from the list of loaded movements using search techniques...
        let movement = find_movement_by_name(&name);

        // create the exercise and add it to exercises
//        let exercise = Exercise { }
    }
    // end of the for each exercise

}

fn find_movement_by_name(name: &str) {
    ()
}

// these workouts have sets and recommended rep ranges,
// but the true completed reps and weights have to be included
// later.  It is just your "program", not what you're actually
// tracking.  They are in the movements.txt file
fn parse_predefined_workout() {
    ()
}
