// TODO
// make readable from text file (some way to input workouts)
// some way to define movements and workouts persistently
// 
mod exercise;
mod parser;

use exercise::*;

fn main() {
    let movements = parser::parse_movements();
    for m in movements {
        println!("{}", m.name);
    }
}
