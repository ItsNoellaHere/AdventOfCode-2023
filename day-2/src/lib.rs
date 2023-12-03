#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub rounds: Vec<Round>,
}

#[derive(Debug)]
pub struct Round {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

pub fn build_game(game: &str) -> Option<Game> {
    let id = game
        .split(":")
        .nth(0)?
        .split(" ")
        .nth(1)?
        .parse::<usize>()
        .unwrap_or(0);
    let rounds = game
        .split(":")
        .nth(1)
        .unwrap()
        .split(";")
        .map(|round| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            round.split(",").for_each(|color| {
                let color = color.trim();

                if color.contains("red") {
                    red = color.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
                } else if color.contains("green") {
                    green = color.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
                } else if color.contains("blue") {
                    blue = color.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
                }
            });

            Round { red, green, blue }
        })
        .collect::<Vec<Round>>();

    Some(Game { id, rounds })
}

pub mod part_1 {
    use super::*;

    impl Game {
        pub fn check_game(&self, total: &Round) -> bool {
            for round in &self.rounds {
                if round.red > total.red || round.green > total.green || round.blue > total.blue {
                    return false;
                }
            }

            true
        }
    }

    pub fn sum_of_invalid_games(input: &Vec<String>, total: &Round) -> usize {
        input
            .iter()
            .map(|game| {
                build_game(game).unwrap_or(Game {
                    id: 0,
                    rounds: vec![],
                })
            })
            .filter(|game| game.check_game(total))
            .map(|game| game.id)
            .sum::<usize>()
    }
}

pub mod part_2 {
    use super::*;

    pub fn find_minimum_cubes(game: &Game) -> Round {
        game.rounds.iter().fold(
            Round {
                red: 0,
                green: 0,
                blue: 0,
            },
            |acc, round| Round {
                red: if acc.red < round.red {
                    round.red
                } else {
                    acc.red
                },
                green: if acc.green < round.green {
                    round.green
                } else {
                    acc.green
                },
                blue: if acc.blue < round.blue {
                    round.blue
                } else {
                    acc.blue
                },
            },
        )
    }

    pub fn sum_of_power(input: &Vec<String>) -> usize {
        input.iter().map(|game| {
            build_game(game).unwrap_or(Game {
                id: 0,
                rounds: vec![],
            })
        }).map(|game| {
            let minimum = find_minimum_cubes(&game);
            minimum.red * minimum.green * minimum.blue
        }).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use super::super::part_1::*;
        use super::super::*;

        fn get_total() -> Round {
            Round {
                red: 12,
                green: 13,
                blue: 14,
            }
        }

        #[test]
        fn sample() {
            let input = vec![
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                    .to_string(),
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                    .to_string(),
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
            ];

            let result = sum_of_invalid_games(&input, &get_total());
            assert_eq!(result, 8);
        }

        #[test]
        fn valid_game() {
            let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string();
            let game = build_game(&input).unwrap();
            let total = get_total();
            assert_eq!(game.check_game(&total), true);
        }

        #[test]
        fn invalid_game() {
            let input = "Game 1: 3 blue, 20 red; 1 red, 2 green, 6 blue; 2 green".to_string();
            let game = build_game(&input).unwrap();
            let total = get_total();
            assert_eq!(game.check_game(&total), false);
        }
    }

    mod part_2 {
        use super::super::part_2::*;

        #[test]
        fn sample() {
            let input = vec![
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                    .to_string(),
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                    .to_string(),
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
            ];

            let result = sum_of_power(&input);
            assert_eq!(result, 2286);
        }
    }
}
