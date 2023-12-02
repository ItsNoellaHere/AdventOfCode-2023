#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "1abc2\n
            pqr3stu8vwx\n
            a1b2c3d4e5f\n
            treb7uchet";

        let result = find_calibration_value(input);
        assert_eq!(result, 77);
    }
}
