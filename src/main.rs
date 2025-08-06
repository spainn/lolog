pub mod parser;

use parser::parse_exercises;

use std::str::FromStr;

//macro_rules! string_to_muscle {
//    ($enum_name:identifier, $value:expr, $pattern:pat =>)
//}

enum MovementPattern {
    HorizontalPress,
    Squat,
    HipHinge,
    Row,
    VerticalPull,
    SupinatedCurl,
    PronatedCurl,
    TricepPressdown,
    TricepExtension,
    HamstringCurl,
    ShoulderPress,
    AbIsolation,
    CalfIsolation,
    NeckIsolation,
    ShoulderIsolation
}

#[derive(Debug)]
enum Muscle {
    Chest,
    AnteriorDeltoids,
    LateralDeltoids,
    PosteriorDeltoids,
    Triceps,

    Lats,
    Traps,
    Biceps,
    Forearms,
    LowerBack,

    Quadriceps,
    Hamstrings,
    Glutes,
    Adductors,
    Abductors,
    Calves,

    Abs,
    Obliques,
    Neck,
}

impl FromStr for Muscle {
    // temp
}

enum Measurement {
    Neck(u32),
    Shoulders(u32),
    Chest(u32),
    Waist(u32),
    LeftArm(u32),
    RightArm(u32),
    LeftForearm(u32),
    RightForearm(u32),
    LeftQuad(u32),
    RightQuad(u32),
    LeftCalf(u32),
    RightCalf(u32),
    
    WaistFullExhale(u32),
    LeftArmUnflexed(u32),
    RightArmUnflexed(u32),
    ChestNippleHeight(u32),
}

#[derive(Debug)]
pub struct Exercise {
    name: String,
    main_mover: Muscle,
}

// an exercise and a number of sets
struct ExerciseAndSets(Exercise, u32);

// need to add a way to track volume and add workouts through cmd line

fn main() {
    parse_exercises();
}
