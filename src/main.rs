pub mod parser;

use parser::parse_exercises;

use strum_macros::EnumString;

#[derive(Debug, EnumString)]
#[strum(ascii_case_insensitive)]
enum MovementPattern {
    HorizontalPress,
    OverheadPress,
    Squat,
    Hinge,
    Row,
    VerticalPull,
    SupinatedCurl,
    PronatedCurl,
    TricepPressdown,
    TricepExtension,
    HamstringCurl,
    AbIsolation,
    CalfIsolation,
    NeckIsolation,
    ShoulderIsolation
}

#[derive(Debug, EnumString)]
#[strum(ascii_case_insensitive)]
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
    movement_pattern: MovementPattern,
}

// an exercise and a number of sets
struct ExerciseAndSets(Exercise, u32);

// need to add a way to track volume and add workouts through cmd line

fn main() {
    parse_exercises();

    println!("\n\n\n\n");

    // movement pattern tests
    let test1 = "hinge";
    let test2 = "NECKISOLATION";

    // muscle tests
    let test3 = "cHeSt";
    let test4 = "AnteriorDeltoids";

    let conv1: MovementPattern = test1.parse().unwrap();
    let conv2: MovementPattern = test2.parse().unwrap();

    println!("{:?}", conv1);
    println!("{:?}", conv2);

    let conv3: Muscle = test3.parse().unwrap();
    let conv4: Muscle = test4.parse().unwrap();

    println!("{:?}", conv3);
    println!("{:?}", conv4);
}
