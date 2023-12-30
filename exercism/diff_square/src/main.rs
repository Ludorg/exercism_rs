pub fn square_of_sum(n: u32) -> u32 {
    let mut r = 0;
    for i in 1..=n {
        r += i;
    }
    r = r * r;
    println!("square of sum of 1...{n} is {r}");
    r
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut r = 0;
    for i in 1..=n {
        r += i * i;
    }
    println!("sum of squares of 1...{n} is {r}");
    r
}

pub fn difference(n: u32) -> u32 {
    let r = square_of_sum(n) - sum_of_squares(n);
    println!("difference between square of sum of 1...{n} and sum of squares of 1...{n} is {r}");
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn square_of_sum_1() {
        assert_eq!(1, square_of_sum(1));
    }

    #[test]
    fn square_of_sum_5() {
        assert_eq!(225, square_of_sum(5));
    }

    #[test]
    fn square_of_sum_100() {
        assert_eq!(25_502_500, square_of_sum(100));
    }

    #[test]
    fn sum_of_squares_1() {
        assert_eq!(1, sum_of_squares(1));
    }

    #[test]
    fn sum_of_squares_5() {
        assert_eq!(55, sum_of_squares(5));
    }

    #[test]
    fn sum_of_squares_100() {
        assert_eq!(338_350, sum_of_squares(100));
    }

    #[test]
    fn difference_1() {
        assert_eq!(0, difference(1));
    }

    #[test]
    fn difference_5() {
        assert_eq!(170, difference(5));
    }

    #[test]
    fn difference_100() {
        assert_eq!(25_164_150, difference(100));
    }
}

fn main() {
    assert_eq!(sum_of_squares(10), 385);
    assert_eq!(square_of_sum(10), 3025);
    assert_eq!(difference(10), 2640);
}
