use day_5::part_1;
use day_5::part_2;
use std::fs;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Please provide a filename");
        return;
    }

    let data = read_file(&args[1]);

    println!("Part 1 -> {}", part_1::find_closest_seed(&data));
    println!("Part 2 -> {}", part_2::find_closest_seed(&data));
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
}
