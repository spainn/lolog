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

#[derive(Debug)]
struct Exercise {
    main_mover: Muscle,
    sets: u32,
}

// need to add a way to track volume and add workouts through cmd line

fn main() {
    let bench_press = Exercise { main_mover: Muscle::Chest(String::from("chest")) };
    println!("{:?}", bench_press);

    if let Muscle::Chest(s) = &bench_press.main_mover {
        println!("Main mover is {}", s);
    } else {
        println!("Main mover is not chest.");
    }
}
