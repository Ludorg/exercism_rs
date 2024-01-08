/// The Collatz Conjecture or 3x+1 problem can be summarized as follows:
///
/// Take any positive integer n.
/// If n is even, divide n by 2 to get n / 2.
/// If n is odd, multiply n by 3 and add 1 to get 3n + 1.
/// Repeat the process indefinitely.
/// The conjecture states that no matter which number you start with, you will always reach 1 eventually.
///
/// Given a number n, return the number of steps required to reach 1.
pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n;
    let mut steps = 0;
    loop {
        if n == 0 {
            return None::<u64>;
        }
        if n == 1 {
            break;
        }

        if n % 2 == 0 {
            n /= 2;
        } else if n >= u64::MAX / 3 {
            return None::<u64>;
        } else {
            n = n * 3 + 1;
        }

        steps += 1;
    }

    Some(steps)
}

fn main() {
    assert_eq!(collatz(12), Some(9));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one() {
        assert_eq!(Some(0), collatz(1));
    }

    #[test]
    fn sixteen() {
        assert_eq!(Some(4), collatz(16));
    }

    #[test]
    fn twelve() {
        assert_eq!(Some(9), collatz(12));
    }

    #[test]
    fn one_million() {
        assert_eq!(Some(152), collatz(1_000_000));
    }

    #[test]
    fn zero() {
        assert_eq!(None, collatz(0));
    }

    #[test]
    fn test_110243094271() {
        let val = 110243094271;
        assert_eq!(None, collatz(val));
    }

    #[test]
    fn max_div_3() {
        let max = u64::MAX / 3;
        assert_eq!(None, collatz(max));
    }

    #[test]
    fn max_minus_1() {
        let max = u64::MAX - 1;
        assert_eq!(None, collatz(max));
    }
}
