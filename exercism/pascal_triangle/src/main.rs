pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = vec![];

        for n in 0..row_count {
            let pow_11 = 11_u32.pow(n);
            rows.push(number_to_digits(pow_11));
        }
        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

fn number_to_digits(number: u32) -> Vec<u32> {
    let mut ret = vec![];
    let mut n = number;
    while n != 0 {
        ret.push(n % 10);
        n /= 10;
    }
    ret.reverse();
    ret
}

fn main() {
    println!("digits in num {:?}", number_to_digits(123456));
    println!("digits in num {:?}", number_to_digits(88771974));

    let pt = PascalsTriangle::new(4);
    println!("{:?}", pt.rows());

    let pt = PascalsTriangle::new(8);
    println!("{:?}", pt.rows());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zero_rows() {
        let pt = PascalsTriangle::new(0);
        let expected: Vec<Vec<u32>> = vec![];
        assert_eq!(pt.rows(), expected);
    }

    #[test]
    fn single_row() {
        let pt = PascalsTriangle::new(1);
        let expected: Vec<Vec<u32>> = vec![vec![1]];
        assert_eq!(pt.rows(), expected);
    }

    #[test]
    fn two_rows() {
        let pt = PascalsTriangle::new(2);
        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1]];
        assert_eq!(pt.rows(), expected);
    }

    #[test]
    fn three_rows() {
        let pt = PascalsTriangle::new(3);
        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1]];
        assert_eq!(pt.rows(), expected);
    }

    #[test]
    fn four_rows() {
        let pt = PascalsTriangle::new(4);
        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]];
        assert_eq!(pt.rows(), expected);
    }

    #[test]
    fn five_rows() {
        let pt = PascalsTriangle::new(5);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(pt.rows(), expected);
    }

    #[test]
    fn six_rows() {
        let pt = PascalsTriangle::new(6);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
        ];
        assert_eq!(pt.rows(), expected);
    }

    #[test]
    fn ten_rows() {
        let pt = PascalsTriangle::new(10);
        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
            vec![1, 6, 15, 20, 15, 6, 1],
            vec![1, 7, 21, 35, 35, 21, 7, 1],
            vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
            vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
        ];
        assert_eq!(pt.rows(), expected);
    }
}
