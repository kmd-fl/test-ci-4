pub fn add1(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add1(2, 2);
        assert_eq!(result, 4);
    }
}
