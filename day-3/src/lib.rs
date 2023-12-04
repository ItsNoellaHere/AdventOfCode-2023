#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<'a> Default for &'a Point {
    fn default() -> &'a Point {
        &Point { x: 0, y: 0 }
    }
}

impl Point {
    pub fn is_neighbour(&self, other: &Point) -> bool {
        let x_diff = (self.x as i32 - other.x as i32).abs();
        let y_diff = (self.y as i32 - other.y as i32).abs();
        x_diff <= 1 && y_diff <= 1
    }

    pub fn is_neighbour_of_any(&self, others: &Vec<Point>) -> bool {
        others.iter().any(|other| self.is_neighbour(other))
    }

    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum PartNumber {
    Number(usize),
    Symbol(char),
    #[default]
    None,
}

#[derive(Debug, Default, Clone)]
pub struct Part {
    pub number: PartNumber,
    pub points: Vec<Point>,
}

impl<'a> Default for &'a Part {
    fn default() -> &'a Part {
        static DEFAULT: Part = Part {
            number: PartNumber::None,
            points: vec![],
        };
        &DEFAULT
    }
}

pub fn parse_input(input: &Vec<String>) -> Vec<Part> {
    let mut parts: Vec<Part> = Vec::new();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                let number = c.to_digit(10).unwrap() as usize;
                let mut last_part = parts.last().unwrap_or_default().clone();
                let points = last_part.points.clone();
                let point = Point::new(x, y);

                if points.last().unwrap_or_default().is_neighbour(&point)
                    && match last_part.number {
                        PartNumber::Number(_) => true,
                        _ => false,
                    }
                {
                    last_part.points.push(point);
                    let last_number = match last_part.number {
                        PartNumber::Number(n) => n,
                        _ => 0,
                    };
                    last_part.number = PartNumber::Number(last_number * 10 + number);
                    parts.pop();
                    parts.push(last_part);
                } else {
                    parts.push(Part {
                        number: PartNumber::Number(number),
                        points: vec![Point::new(x, y)],
                    });
                }
            } else if c != '.' {
                parts.push(Part {
                    number: PartNumber::Symbol(c),
                    points: vec![Point::new(x, y)],
                });
            }
        }
    }
    parts
}

pub mod part_1 {
    use super::*;

    pub fn number_has_neighbor_symbol(number: &Part, symbols: &Vec<Part>) -> bool {
        number.points.iter().any(|point| {
            point.is_neighbour_of_any(&symbols.iter().flat_map(|s| s.points.clone()).collect())
        })
    }

    pub fn sum_of_part_numbers(input: &Vec<String>) -> usize {
        let parts = parse_input(input);
        let symbols: Vec<Part> = parts
            .iter()
            .filter(|p| match p.number {
                PartNumber::Symbol(_) => true,
                _ => false,
            })
            .map(|p| p.clone())
            .collect();
        parts.iter().filter(|p| match p.number { PartNumber::Number(_) => true, _ => false } && number_has_neighbor_symbol(p, &symbols)).map(|p| match p.number {
            PartNumber::Number(n) => n,
            _ => 0,
        }).sum()
    }
}

pub mod part_2 {
    use super::*;

    pub fn gear_has_two_neighbor_number(gear: &Part, numbers: &Vec<Part>) -> bool {
        numbers
            .iter()
            .filter(|n| gear.points.iter().any(|point| point.is_neighbour_of_any(&n.points)))
            .count()
            == 2
    }

    pub fn find_gear_ratio(gear: &Part, numbers: &Vec<Part>) -> usize {
        numbers.iter().filter(|n| gear.points.iter().any(|point| point.is_neighbour_of_any(&n.points))).map(|n| match n.number {
            PartNumber::Number(n) => n,
            _ => 1,
        }).product()
    }

    pub fn sum_of_gear_ratios(input: &Vec<String>) -> usize {
        let parts = parse_input(input);
        let numbers: Vec<Part> = parts
            .iter()
            .filter(|p| match p.number {
            PartNumber::Number(_) => true,
            _ => false,
            })
            .map(|p| p.clone())
            .collect();
        let gears: Vec<Part> = parts
            .iter()
            .filter(|p| match p.number {
                PartNumber::Symbol('*') => true,
                _ => false,
            })
            .map(|p| p.clone())
            .filter(|p| gear_has_two_neighbor_number(p, &numbers))
            .collect();
        gears.iter().map(|g| find_gear_ratio(g, &numbers)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_is_neighbour() {
        let point = Point::new(0, 0);
        let other = Point::new(1, 1);
        assert!(point.is_neighbour(&other));
    }

    #[test]
    fn point_is_not_neighbour() {
        let point = Point::new(0, 0);
        let other = Point::new(2, 2);
        assert!(!point.is_neighbour(&other));
    }

    #[test]
    fn any_is_neighbour() {
        let point = Point::new(0, 0);
        let others = vec![Point::new(1, 1), Point::new(2, 2)];
        assert!(point.is_neighbour_of_any(&others));
    }

    #[test]
    fn any_is_not_neighbor() {
        let point = Point::new(0, 0);
        let others = vec![Point::new(2, 2), Point::new(3, 3)];
        assert!(!point.is_neighbour_of_any(&others));
    }

    mod part_1 {
        use super::super::part_1::*;
        use super::super::*;

        #[test]
        fn sample() {
            let input = vec![
                "467..114..".to_string(),
                "...*......".to_string(),
                "..35..633.".to_string(),
                "......#...".to_string(),
                "617*......".to_string(),
                ".....+.58.".to_string(),
                "..592.....".to_string(),
                "......755.".to_string(),
                "...$.*....".to_string(),
                ".664.598..".to_string(),
            ];
            let result = sum_of_part_numbers(&input);

            assert_eq!(result, 4361);
        }

        #[test]
        fn single_number_input() {
            let input = vec!["..592.....".to_string()];
            let result = parse_input(&input);

            println!("{:?}", result);
            assert_eq!(result.len(), 1);
            assert_eq!(result.first().unwrap().number, PartNumber::Number(592));
            assert_eq!(result.first().unwrap().points.len(), 3);
            assert_eq!(
                result.first().unwrap().points.first().unwrap(),
                &Point::new(2, 0)
            );
            assert_eq!(
                result.first().unwrap().points.last().unwrap(),
                &Point::new(4, 0)
            );
        }

        #[test]
        fn single_symbol_input() {
            let input = vec!["..$.......".to_string()];
            let result = parse_input(&input);

            println!("{:?}", result);
            assert_eq!(result.len(), 1);
            assert_eq!(result.first().unwrap().number, PartNumber::Symbol('$'));
            assert_eq!(result.first().unwrap().points.len(), 1);
            assert_eq!(
                result.first().unwrap().points.first().unwrap(),
                &Point::new(2, 0)
            );
        }

        #[test]
        fn multi_symbol_input() {
            let input = vec!["..$....#..".to_string()];
            let result = parse_input(&input);

            println!("{:?}", result);
            assert_eq!(result.len(), 2);
            assert_eq!(result.first().unwrap().number, PartNumber::Symbol('$'));
            assert_eq!(result.first().unwrap().points.len(), 1);
            assert_eq!(
                result.first().unwrap().points.first().unwrap(),
                &Point::new(2, 0)
            );
            assert_eq!(result.last().unwrap().number, PartNumber::Symbol('#'));
            assert_eq!(result.last().unwrap().points.len(), 1);
            assert_eq!(
                result.last().unwrap().points.first().unwrap(),
                &Point::new(7, 0)
            );
        }

        #[test]
        fn symbol_and_number_input() {
            let input = vec!["..$....4..".to_string()];
            let result = parse_input(&input);

            println!("{:?}", result);
            assert_eq!(result.len(), 2);
            assert_eq!(result.first().unwrap().number, PartNumber::Symbol('$'));
            assert_eq!(result.first().unwrap().points.len(), 1);
            assert_eq!(
                result.first().unwrap().points.first().unwrap(),
                &Point::new(2, 0)
            );
            assert_eq!(result.last().unwrap().number, PartNumber::Number(4));
            assert_eq!(result.last().unwrap().points.len(), 1);
            assert_eq!(
                result.last().unwrap().points.first().unwrap(),
                &Point::new(7, 0)
            );
        }

        #[test]
        fn one_neighbor() {
            let input = vec!["467*......".to_string()];
            let result = sum_of_part_numbers(&input);

            assert_eq!(result, 467);
        }
    }

    mod part_2 {
        use super::super::part_2::*;
        use super::super::*;

        #[test]
        fn sample() {
            let input = vec![
                "467..114..".to_string(),
                "...*......".to_string(),
                "..35..633.".to_string(),
                "......#...".to_string(),
                "617*......".to_string(),
                ".....+.58.".to_string(),
                "..592.....".to_string(),
                "......755.".to_string(),
                "...$.*....".to_string(),
                ".664.598..".to_string(),
            ];
            let result = sum_of_gear_ratios(&input);

            assert_eq!(result, 467835);
        }

        #[test]
        fn single_gear_input() {
            assert_eq!(
                gear_has_two_neighbor_number(
                    &Part {
                        number: PartNumber::Symbol('*'),
                        points: vec![Point::new(1, 0)]
                    },
                    &vec![
                        Part {
                            number: PartNumber::Number(2),
                            points: vec![Point::new(0, 0)]
                        },
                        Part {
                            number: PartNumber::Number(2),
                            points: vec![Point::new(2, 0)]
                        }
                    ]
                ),
                true
            );
        }

        #[test]
        fn single_gear_ratio() {
            assert_eq!(
                find_gear_ratio(
                    &Part {
                        number: PartNumber::Symbol('*'),
                        points: vec![Point::new(1, 0)]
                    },
                    &vec![
                        Part {
                            number: PartNumber::Number(2),
                            points: vec![Point::new(0, 0)]
                        },
                        Part {
                            number: PartNumber::Number(2),
                            points: vec![Point::new(2, 0)]
                        }
                    ]
                ),
                4
            );
        }
    }
}
