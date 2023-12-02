pub mod part_1 {
    pub fn handel_line(line: &str) -> u32 {
        let nums_in_line: Vec<u32> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();

        if nums_in_line.len() == 0 {
            return 0;
        }

        let first = nums_in_line.first().unwrap();
        let last = nums_in_line.last().unwrap();

        first * 10 + last
    }

    pub fn find_calibration_value(input: &Vec<String>) -> u32 {
        let mut result = 0;

        for line in input.iter() {
            result += handel_line(line);
        }

        result
    }
}

pub mod part_2 {
    fn slice_to_string(input: &[char]) -> String {
        let mut result = String::new();
        for c in input {
            result.push(*c);
        }
        result
    }

    fn replace_didgets(line: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = line.chars().collect();

        for i in 0..chars.len() {
            let slice = slice_to_string(&chars[i..]);
            if slice.starts_with("one") {
                result.push_str("1");
            } else if slice.starts_with("two") {
                result.push_str("2");
            } else if slice.starts_with("three") {
                result.push_str("3");
            } else if slice.starts_with("four") {
                result.push_str("4");
            } else if slice.starts_with("five") {
                result.push_str("5");
            } else if slice.starts_with("six") {
                result.push_str("6");
            } else if slice.starts_with("seven") {
                result.push_str("7");
            } else if slice.starts_with("eight") {
                result.push_str("8");
            } else if slice.starts_with("nine") {
                result.push_str("9");
            } else if chars[i].is_digit(10) {
                result.push(chars[i]);
            }
        }
        result
    }

    pub fn find_calibration_value(input: &Vec<String>) -> u32 {
        let input: Vec<String> = input.iter().map(|l| replace_didgets(&l)).collect();
        let mut result = 0;

        for line in input.iter() {
            result += super::part_1::handel_line(&line);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use super::super::part_1::*;

        #[test]
        fn sample() {
            let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let result = find_calibration_value(&input);
            assert_eq!(result, 142);
        }

        #[test]
        fn none() {
            let input = vec!["trebuchet"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let result = find_calibration_value(&input);
            assert_eq!(result, 0);
        }

        #[test]
        fn one() {
            let input = vec!["treb7uchet"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let result = find_calibration_value(&input);
            assert_eq!(result, 77);
        }

        #[test]
        fn two_single() {
            let input = vec!["pqr3stu8vwx"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let result = find_calibration_value(&input);
            assert_eq!(result, 38);
        }

        #[test]
        fn two_many() {
            let input = vec!["1abc2", "pqr3stu8vwx"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let result = find_calibration_value(&input);
            assert_eq!(result, 50);
        }

        #[test]
        fn many_many() {
            let input = vec!["a1b2c3d4e5f", "a1b2c34e5f"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let result = find_calibration_value(&input);
            assert_eq!(result, 30);
        }
    }

    mod part_2 {
        use super::super::part_2::*;

        #[test]
        fn sample() {
            let input = vec![
                "two1nine",
                "eightwothree",
                "abcone2threexyz",
                "xtwone3four",
                "4nineeightseven2",
                "zoneight234",
                "7pqrstsixteen",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

            let result = find_calibration_value(&input);
            assert_eq!(result, 281);
        }

        #[test]
        fn empty() {
            let input: Vec<String> = vec![];

            let result = find_calibration_value(&input);
            assert_eq!(result, 0);
        }

        #[test]
        fn none() {
            let input = vec!["trebuchet"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let result = find_calibration_value(&input);
            assert_eq!(result, 0);
        }

        #[test]
        fn one_num() {
            let input = vec!["treb7uchet"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let result = find_calibration_value(&input);
            assert_eq!(result, 77);
        }

        #[test]
        fn single_duplicate() {
            let input = vec!["threeightwothree"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let result = find_calibration_value(&input);
            assert_eq!(result, 33);
        }

        #[test]
        fn one() {
            let input = vec!["one"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let result = find_calibration_value(&input);
            assert_eq!(result, 11);
        }

        #[test]
        fn overlap() {
            let input = vec!["oneight"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let result = find_calibration_value(&input);
            assert_eq!(result, 18);
        }
    }
}
