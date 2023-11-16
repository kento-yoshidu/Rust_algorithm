// https://atcoder.jp/contests/abc085/tasks/abc085_c

pub fn run(n: isize, y: isize) -> Vec<isize> {
    for i in 0..=n {
        for j in 0..=n {
            let k = n - i - j;

            if k < 0 || n < k {
                continue;
            }

            if i * 10000 + j * 5000 + k * 1000 == y {
                return vec![i, j, k];
            }
        }
    }

    vec![-1, -1, -1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![0, 9, 0], run(9, 45000));
        assert_eq!(vec![-1, -1, -1], run(20, 196000));
        assert_eq!(vec![2, 54, 944], run(1000, 1234000));
        assert_eq!(vec![2000, 0, 0], run(2000, 20000000));
    }
}
