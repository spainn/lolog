pub struct Workout {
    pub name: String,
    pub date: String,
    pub exercises: Vec<Exercise>,
}

pub struct Exercise {
    pub movement: Movement,
    pub position: u32,
    pub num_sets: u32,
    pub sets: Vec<Set>,
}

impl Exercise {
    pub fn weight_moved(&self) -> u32 {
        let mut weight_moved = 0;

        for set in &self.sets {
            weight_moved += set.reps * set.weight;
        }

        weight_moved
    }
}

pub struct Movement {
    pub name: String,
    pub alias: String,
    pub main_mover: String,
    pub default_to_failure: bool,
}

pub struct Set {
    pub position: u32,
    pub reps: u32,
    pub weight: u32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_weight_moved() {
        let bench_press = Movement {
            name: "bench press".to_string(),
            alias: "bp".to_string(),
            main_mover: "chest".to_string(),
            default_to_failure: false,
        };

        let set1 = Set {
            position: 0,
            reps: 6,
            weight: 225,
        };

        let set2 = Set {
            position: 1,
            reps: 5,
            weight: 225,
        };

        let set3 = Set {
            position: 2,
            reps: 4,
            weight: 225,
        };

        let sets: Vec<Set> = vec![set1, set2, set3];

        let exercise1 = Exercise {
            movement: bench_press,
            position: 0,
            num_sets: sets.len() as u32,
            sets: sets,
        };

        // run actual test
        assert_eq!(exercise1.weight_moved(), 3375);
    }
}
