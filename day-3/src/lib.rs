pub mod part_1 {
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }
}


#[cfg(test)]
mod tests {
    mod part_1 {
        use super::super::part_1::*;

        #[test]
        fn it_works() {
            let result = add(2, 2);
            assert_eq!(result, 4);
        }
    }
}
