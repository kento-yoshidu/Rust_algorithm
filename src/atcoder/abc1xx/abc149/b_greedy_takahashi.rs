// https://atcoder.jp/contests/abc149/tasks/abc149_b

pub fn run(a: usize, b: usize, k: usize) -> [usize; 2] {
    if a >= k {
        [a - k, b]
    } else if  k >= a + b {
        [0, 0]
    } else {
        [0, b - (k - a)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![0, 2], run(2, 3, 3));
        assert_eq!(vec![0, 0], run(500000000000, 500000000000, 1000000000000));
        assert_eq!(vec![0, 0], run(500000000000, 499999999999, 1000000000000));
    }
}
