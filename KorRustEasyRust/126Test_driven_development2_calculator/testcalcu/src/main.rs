fn math(input: &str) -> i32 {
    9
}
// "1 + 1" -> 2

#[cfg(test)]
mod tests {
    use super::math; // super = the space just above

    #[test]
    fn one_plus_one_is_two() {
        assert_eq!(math("1 + 1"), 2);
    }
    #[test]
    fn one_minus_two_is_minus_one() {
        assert_eq!(math("1 - 2"), -1);
    }
    #[test]
    fn one_minus_two_is_minus_two() {
        assert_eq!(math("1 - -1"), 2);
    }
}

fn main() {}
