/*
Given an integer n, return a string array answer (1-indexed) where:
    answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
    answer[i] == "Fizz" if i is divisible by 3.
    answer[i] == "Buzz" if i is divisible by 5.
    answer[i] == i (as a string) if none of the above conditions are true.
*/

// Cow<'static, str>

mod tests;

fn fizz_buzz(the_number: i64) -> String {
    if the_number % 15 == 0 {
        "FizzBuzz".to_string()
    } else if the_number % 3 == 0 {
        "Fizz".to_string()
    } else if the_number % 5 == 0 {
        "Buzz".to_string()
    } else {
        the_number.to_string()
    }
}

fn main() {
    println!("{}", fizz_buzz(46));
}