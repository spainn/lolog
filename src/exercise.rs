pub struct Exercise {
    pub movement: Movement,
    pub position: u32,
    pub num_sets: u32,
    pub sets: Vec<Set>,
}

pub struct Movement {
    pub name: String,
    pub main_mover: String,
    pub default_to_failure: bool,
}

pub struct Set {
    pub position: u32,
    pub reps: u32,
    pub weight: u32,
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
