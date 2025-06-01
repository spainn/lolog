struct Exercise {
    movement: Movement,
    position: u32,
    num_sets: u32,
    sets: Vec<Set>,
}

struct Movement {
    name: String,
    main_mover: String,
    default_to_failure: bool,
}

struct Set {
    position: u32,
    reps: u32,
    weight: u32,
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
