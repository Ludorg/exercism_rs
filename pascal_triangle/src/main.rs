pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = vec![];

        for i in 0..row_count {
            let mut row: Vec<u32> = vec![];
            for j in 0..=i {
                row.push(binomial_coefficient(i, j));
            }
            rows.push(row);
        }
        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
pub fn factorial(num: u64) -> u64 {
    (1..=num).product()
}

fn binomial_coefficient(n: u32, k: u32) -> u32 {
    (factorial(n as u64) / (factorial(k as u64) * factorial((n - k) as u64))) as u32
}
fn main() {
    println!("factorial(6)={}", factorial(6));
    println!("factorial(1)={}", factorial(1));
    println!("factorial(4)={}", factorial(4));
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
