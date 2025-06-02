use std::fs;

use crate::exercise::*;

pub fn parse_movements() -> Vec<Movement> {
    const MOVEMENTS_FILE: &str = "movements.txt";

    println!("In file {MOVEMENTS_FILE}");

    let contents = fs::read_to_string(MOVEMENTS_FILE)
        .expect("should have been able ot read this file");
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
                    println!("Movements line after parsing: {line}");
                },
                _ => {}
            }
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
            _ => true
        }
    }
}

//fn parse_movement_line(line: &str) -> Movement {
//    let no_whitespace: String = line.chars().filter(|c| !c.is_whitespace()).collect();
//    let args: Vec<&str> = no_whitespace.split(',').collect();
//
//    Movement {
//        name: args[0].to_string(),
//        alias: args[1].to_string(),
//        main_mover: args[2].to_string(),
//        default_to_failure: match args[3] {
//            "true" => true,
//            "t" => true,
//            "True" => true,
//            "T" => true,
//            "false" => false,
//            "f" => false,
//            "False" => false,
//            "F" => false,
//            _ => true
//        }
//    }
//}
