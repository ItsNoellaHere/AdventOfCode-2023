pub struct Card {
    pub card_number: usize,
    pub winning_numbers: Vec<usize>,
    pub playing_numbers: Vec<usize>,
}

impl Card {
    fn new(card_number: usize, winning_numbers: Vec<usize>, playing_numbers: Vec<usize>) -> Card {
        Card {
            card_number,
            winning_numbers,
            playing_numbers,
        }
    }
}

fn parse_input(input: &Vec<String>) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for line in input {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(":");
        let card_number = parts
            .next()
            .unwrap()
            .split_at(4)
            .1
            .trim()
            .parse::<usize>()
            .unwrap();
        let mut numbers = parts.next().unwrap().trim().split("|");
        let winning_numbers = numbers
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .filter_map(|s| match s.parse::<usize>() {
                Ok(n) => Some(n),
                Err(_) => None,
            })
            .collect::<Vec<usize>>();
        let playing_numbers = numbers
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .filter_map(|s| match s.parse::<usize>() {
                Ok(n) => Some(n),
                Err(_) => None,
            })
            .collect::<Vec<usize>>();
        cards.push(Card::new(card_number, winning_numbers, playing_numbers))
    }
    cards
}

pub mod part_1 {
    use super::*;

    pub fn check_winning_numbers(card: &Card) -> usize {
        let count = card.winning_numbers
            .iter()
            .filter(|n| card.playing_numbers.contains(n))
            .count();
        if count == 0 {
            return 0;
        }
        let two: usize = 2;
        (two).pow(count as u32 - 1)
    }

    // Card #: 5 winning | 8 playing
    pub fn sum_of_points(input: &Vec<String>) -> usize {
        let cards = parse_input(input);
        cards
            .iter()
            .map(|card| check_winning_numbers(card))
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input() {
        let input = vec!["Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string()];
        let result = super::parse_input(&input);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].card_number, 1);
        assert_eq!(result[0].winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(
            result[0].playing_numbers,
            vec![83, 86, 6, 31, 17, 9, 48, 53]
        );
    }

    mod part_1 {
        use super::super::*;
        use super::super::part_1::*;

        #[test]
        fn sample() {
            let input = vec![
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
                "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
                "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
                "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
                "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
                "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
            ];
            let result = sum_of_points(&input);
            assert_eq!(result, 13);
        }

        #[test]
        fn zero_points() {
            let input = Card::new(1, vec![31, 18, 13, 56, 72], vec![74, 77, 10, 23, 35, 67, 36, 11]);
            let result = check_winning_numbers(&input);

            assert_eq!(result, 0);
        }

        #[test]
        fn one_point() {
            let input = Card::new(4, vec![41, 92, 73, 84, 69], vec![59, 84, 76, 51, 58, 5, 54, 83]);
            let result = check_winning_numbers(&input);

            assert_eq!(result, 1);
        }

        #[test]
        fn two_points() {
            let input = Card::new(2, vec![13, 32, 20, 16, 61], vec![61, 30, 68, 82, 17, 32, 24, 19]);
            let result = check_winning_numbers(&input);

            assert_eq!(result, 2);
        }
    }
}
