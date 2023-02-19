#[cfg(test)]
mod tests {
    use crate::fizz_buzz;
    #[test]
    fn test_fizz_buzz_fizzbuzz() {
        assert_eq!(fizz_buzz(15), "FizzBuzz");
    }
    #[test]
    fn test_fizz_buzz_fizz() {
        assert_eq!(fizz_buzz(3), "Fizz");
    }
    #[test]
    fn test_fizz_buzz_buzz() {
        assert_eq!(fizz_buzz(5), "Buzz");
    }
    #[test]
    fn test_fizz_buzz_noreturn() {
        assert_eq!(fizz_buzz(46), "46")
    }
}