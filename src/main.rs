mod exercise;

use exercise::*;

fn main() {
    let bench_press = Movement {
        name: "bench press".to_string(),
        main_mover: "chest".to_string(),
        default_to_failure: false,
    };

    let set1 = Set {
        position: 0,
        reps: 6,
        weight: 225
    };


    let set2 = Set {
        position: 1,
        reps: 5,
        weight: 225
    };
    
    let set3 = Set {
        position: 2,
        reps: 4,
        weight: 225
    };

//    let sets: Vec<Set> = Vec::new();
//    sets.push(set1);
//    sets.push(set2);
//    sets.push(set3);
    
    let sets: Vec<Set> = vec!(set1, set2, set3);

    let exercise1 = Exercise {
        movement: bench_press,
        position: 0,
        num_sets: sets.len() as u32,
        sets: sets
    };

    // calculate total weight moved
    println!("Total weight moved: {}", exercise1.weight_moved());
}
