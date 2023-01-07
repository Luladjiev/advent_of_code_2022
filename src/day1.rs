use std::cmp::Ordering;

struct Elf {
    idx: usize,
    calories: i32,
}

pub fn run() {
    let file = "data/day1.txt";
    let content = std::fs::read_to_string(file).expect(&format!("Cannot read file {}", file));

    let mut elves = Vec::<i32>::new();
    let mut calories = 0;

    for line in content.lines() {
        if line.is_empty() {
            elves.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<i32>().unwrap();
        }
    }

    let elf1 = get_max_elf_calories(elves.as_slice(), Vec::new()).unwrap();

    println!(
        "Elf #{} is holding most calories: {}",
        elf1.idx, elf1.calories
    );

    let elf2 = get_max_elf_calories(elves.as_slice(), vec![elf1.idx]).unwrap();
    let elf3 = get_max_elf_calories(elves.as_slice(), vec![elf1.idx, elf2.idx]).unwrap();

    println!("Sum of the calories of top 3 elves is {}", elf1.calories + elf2.calories + elf3.calories);
}

fn get_max_elf_calories(elves: &[i32], skip_elves: Vec<usize>) -> Option<Elf> {
    let iter = elves.iter().enumerate();

    iter.max_by(|(idx_a, val_a), (idx_b, val_b)| {
        if skip_elves.contains(idx_a) {
            return Ordering::Less;
        }

        if skip_elves.contains(idx_b) {
            return Ordering::Greater;
        }

        val_a.cmp(val_b)
    })
    .map(|(index, value)| Elf {
        idx: index,
        calories: *value,
    })
}
