use day_2::part_1;
use day_2::part_2;
use std::fs;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Please provide a filename");
        return;
    }

    let data = read_file(&args[1]);
    let total = day_2::Round {
        red: 12,
        green: 13,
        blue: 14,
    };

    println!("Part 1 -> {}", part_1::sum_of_invalid_games(&data, &total));
    println!("Part 2 -> {}", part_2::sum_of_power(&data));
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}
