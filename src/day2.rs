enum Outcome {
    Win,
    Lose,
    Draw,
}

pub fn run() {
    let file = "data/day2.txt";
    let content = std::fs::read_to_string(file).expect(&format!("Cannot open file {}", file));

    let mut sum = 0;

    for line in content.lines() {
        let result: Vec<&str> = line.split(" ").collect();

        sum += calculate_outcome(result);
    }

    println!("Outcome is {}", sum);
}

fn calculate_outcome(result: Vec<&str>) -> i32 {
    let shape_score = match result[1] {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        &_ => panic!("Invalid shape"),
    };

    let outcome = get_outcome(result);

    return match outcome {
        Outcome::Win => 6 + shape_score,
        Outcome::Draw => 3 + shape_score,
        Outcome::Lose => 0 + shape_score,
    };
}

fn get_outcome(result: Vec<&str>) -> Outcome {
    if result[0].eq("A") && result[1].eq("Z") {
        return Outcome::Lose;
    }

    if result[0].eq("B") && result[1].eq("X") {
        return Outcome::Lose;
    }

    if result[0].eq("C") && result[1].eq("Y") {
        return Outcome::Lose;
    }

    if result[0].eq("A") && result[1].eq("Y") {
        return Outcome::Win;
    }

    if result[0].eq("B") && result[1].eq("Z") {
        return Outcome::Win;
    }

    if result[0].eq("C") && result[1].eq("X") {
        return Outcome::Win;
    }

    return Outcome::Draw;
}
