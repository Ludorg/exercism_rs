/// Determines whether the supplied string is a valid ISBN number
///
/// The ISBN-10 format is 9 digits (0 to 9) plus one check character (either a digit or an X only).
/// In the case the check character is an X, this represents the value '10'.
/// These may be communicated with or without hyphens, and can be checked for their validity by the following formula:
/// (d₁ * 10 + d₂ * 9 + d₃ * 8 + d₄ * 7 + d₅ * 6 + d₆ * 5 + d₇ * 4 + d₈ * 3 + d₉ * 2 + d₁₀ * 1) mod 11 == 0
/// If the result is 0, then it is a valid ISBN-10, otherwise it is invalid.
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut digits: Vec<u32> = vec![];
    for c in isbn.chars() {
        if c.is_ascii_digit() {
            digits.push(c.to_digit(10).unwrap())
        } else if c == 'X' {
            digits.push(10);
            if digits.len() != 10 {
                // check X is wrongly placed
                return false;
            }
        }
    }
    if digits.len() != 10 {
        return false;
    }
    ((digits[0] * 10
        + digits[1] * 9
        + digits[2] * 8
        + digits[3] * 7
        + digits[4] * 6
        + digits[5] * 5
        + digits[6] * 4
        + digits[7] * 3
        + digits[8] * 2
        + digits[9])
        % 11)
        == 0
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn valid() {
        assert!(is_valid_isbn("3-598-21508-8"));
    }

    #[test]
    fn invalid_check_digit() {
        assert!(!is_valid_isbn("3-598-21508-9"));
    }

    #[test]
    fn valid_check_digit_of_10() {
        assert!(is_valid_isbn("3-598-21507-X"));
    }

    #[test]
    fn invalid_character_as_check_digit() {
        assert!(!is_valid_isbn("3-598-21507-A"));
    }

    #[test]
    fn invalid_character_in_isbn() {
        assert!(!is_valid_isbn("3-598-P1581-X"));
    }

    #[test]
    #[allow(non_snake_case)]
    fn invalid_isbn_with_invalid_X() {
        assert!(!is_valid_isbn("3-598-2X507-9"));
    }

    #[test]
    fn valid_isbn_without_dashes() {
        assert!(is_valid_isbn("3598215088"));
    }

    #[test]
    #[allow(non_snake_case)]
    fn valid_isbn_without_dashes_and_X_as_check() {
        assert!(is_valid_isbn("359821507X"));
    }

    #[test]
    fn invalid_isbn_without_dashes_and_no_check_digit() {
        assert!(!is_valid_isbn("359821507"));
    }

    #[test]
    fn invalid_isbn_without_dashes_and_too_long() {
        assert!(!is_valid_isbn("3598215078X"));
    }

    #[test]
    fn too_short_isbn() {
        assert!(!is_valid_isbn("00"));
    }

    #[test]
    fn invalid_isbn_without_check_digit() {
        assert!(!is_valid_isbn("3-598-21507"));
    }

    #[test]
    fn valid_digits_invalid_length() {
        assert!(!is_valid_isbn("35982150881"));
    }

    #[test]
    fn special_characters() {
        assert!(!is_valid_isbn("!@#%!@"));
    }

    #[test]
    #[allow(non_snake_case)]
    fn invalid_isbn_with_check_digit_X_instead_of_0() {
        assert!(!is_valid_isbn("3-598-21515-X"));
    }

    #[test]
    fn empty_isbn() {
        assert!(!is_valid_isbn(""));
    }

    #[test]
    fn input_is_9_characters() {
        assert!(!is_valid_isbn("134456729"));
    }

    #[test]
    fn invalid_characters_are_not_ignored() {
        assert!(!is_valid_isbn("3132P34035"));
    }

    #[test]
    fn too_long_but_contains_a_valid_isbn() {
        assert!(!is_valid_isbn("98245726788"));
    }
}
