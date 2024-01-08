// An Armstrong number is a number that is the sum of its own digits
// each raised to the power of the number of digits.

// For example:

// 9 is an Armstrong number, because 9 = 9^1 = 9
// 10 is not an Armstrong number, because 10 != 1^2 + 0^2 = 1
// 153 is an Armstrong number, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
// 154 is not an Armstrong number, because: 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190

pub fn is_armstrong_number(num: u32) -> bool {
    let n_digits: u64 = (num.checked_ilog10().unwrap_or(0) + 1) as u64;

    println!("{num} has {n_digits} digits");

    let mut number = num as u64;
    let mut sum = 0;

    while number > 0 {
        println!("current digit is {}", number % 10);
        sum += (number % 10).pow(n_digits as u32);
        number = number / 10;
    }

    println!("{num} is an armstrong number: {}", sum == num as u64);
    sum == num as u64
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn zero_is_an_armstrong_number() {
        assert!(is_armstrong_number(0))
    }
    #[test]
    fn single_digit_numbers_are_armstrong_numbers() {
        assert!(is_armstrong_number(5))
    }
    #[test]
    fn there_are_no_2_digit_armstrong_numbers() {
        assert!(!is_armstrong_number(10))
    }
    #[test]
    fn three_digit_armstrong_number() {
        assert!(is_armstrong_number(153))
    }
    #[test]
    fn three_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(100))
    }
    #[test]
    fn four_digit_armstrong_number() {
        assert!(is_armstrong_number(9474))
    }
    #[test]
    fn four_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(9475))
    }
    #[test]
    fn seven_digit_armstrong_number() {
        assert!(is_armstrong_number(9_926_315))
    }
    #[test]
    fn seven_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(9_926_316))
    }
    #[test]
    fn nine_digit_armstrong_number() {
        assert!(is_armstrong_number(912_985_153));
    }
    #[test]
    fn nine_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(999_999_999));
    }
    #[test]
    fn ten_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(3_999_999_999));
    }
    // The following number has an Armstrong sum equal to 2^32 plus itself,
    // and therefore will be detected as an Armstrong number if you are
    // incorrectly using wrapping arithmetic.
    #[test]
    fn properly_handles_overflow() {
        assert!(!is_armstrong_number(4_106_098_957));
    }
}
fn main() {
    println!("{}", is_armstrong_number(9));
    println!("{}", is_armstrong_number(10));
    println!("{}", is_armstrong_number(153));
}
