fn is_prime(n: u32) -> bool {
    let limit = (n as f64).sqrt() as u32;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }

    true
}

static mut prime_list: Vec<u32> = vec![];

fn init_prime_list() {
    for n in 2..=1_000_000 {
        if is_prime(n) {
            unsafe {
                prime_list.push(n);
            }
            println!("{n} is prime")
        }
    }
}

pub fn nth(n: u32) -> u32 {
    unsafe {
        if prime_list.is_empty() {
            init_prime_list();
        }
    }
    println!("What is the 0-indexed {n}th prime number?");
    let r: u32;
    unsafe {
        r = prime_list[n as usize];
    }
    r
}

mod tests {

    use super::*;
    #[test]
    fn first_prime() {
        assert_eq!(nth(0), 2);
    }

    #[test]
    fn second_prime() {
        assert_eq!(nth(1), 3);
    }

    #[test]
    fn sixth_prime() {
        assert_eq!(nth(5), 13);
    }

    #[test]
    fn big_prime() {
        assert_eq!(nth(10_000), 104_743);
    }
}

fn main() {
    println!("Hello, world!");
}
