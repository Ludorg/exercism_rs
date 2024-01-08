pub fn factors(n: u64) -> Vec<u64> {
    let mut ret: Vec<u64> = vec![];
    let mut n = n;

    while n % 2 == 0 {
        ret.push(2);
        n = n / 2;
    }

    let sqrt_n = f64::sqrt(n as f64) as u64;
    for i in (3..=sqrt_n).step_by(2) {
        while n % i == 0 {
            ret.push(i);
            n = n / i;
        }
    }

    if n > 2 {
        ret.push(n);
    }

    ret
}

fn main() {
    println!("{:?}", factors(315));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_factors() {
        let factors = factors(1);
        let expected = [];
        assert_eq!(factors, expected);
    }

    #[test]
    fn prime_number() {
        let factors = factors(2);
        let expected = [2];
        assert_eq!(factors, expected);
    }

    #[test]
    fn another_prime_number() {
        let factors = factors(3);
        let expected = [3];
        assert_eq!(factors, expected);
    }

    #[test]
    fn square_of_a_prime() {
        let factors = factors(9);
        let expected = [3, 3];
        assert_eq!(factors, expected);
    }

    #[test]
    fn product_of_first_prime() {
        let factors = factors(4);
        let expected = [2, 2];
        assert_eq!(factors, expected);
    }

    #[test]
    fn cube_of_a_prime() {
        let factors = factors(8);
        let expected = [2, 2, 2];
        assert_eq!(factors, expected);
    }

    #[test]
    fn product_of_second_prime() {
        let factors = factors(27);
        let expected = [3, 3, 3];
        assert_eq!(factors, expected);
    }

    #[test]
    fn product_of_third_prime() {
        let factors = factors(625);
        let expected = [5, 5, 5, 5];
        assert_eq!(factors, expected);
    }

    #[test]
    fn product_of_first_and_second_prime() {
        let factors = factors(6);
        let expected = [2, 3];
        assert_eq!(factors, expected);
    }

    #[test]
    fn product_of_primes_and_non_primes() {
        let factors = factors(12);
        let expected = [2, 2, 3];
        assert_eq!(factors, expected);
    }

    #[test]
    fn product_of_primes() {
        let factors = factors(901255);
        let expected = [5, 17, 23, 461];
        assert_eq!(factors, expected);
    }

    #[test]
    fn factors_include_a_large_prime() {
        let factors = factors(93819012551);
        let expected = [11, 9539, 894119];
        assert_eq!(factors, expected);
    }
}
