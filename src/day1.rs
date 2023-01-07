pub fn run() {
    let file = "data/day1.txt";
    let content = std::fs::read_to_string(file).expect(&format!("Cannot read file {}", file));

    let mut elves = Vec::new();
    let mut calories = 0;

    for line in content.lines() {
        if line.is_empty() {
            elves.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<i32>().unwrap();
        }
    }

    let result = elves
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(_, index)| index);

    println!("Elf #{:?} is holding most calories", result.unwrap());
}
