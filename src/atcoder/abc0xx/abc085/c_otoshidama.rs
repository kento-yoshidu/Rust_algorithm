// https://atcoder.jp/contests/abc085/tasks/abc085_c

fn run(n: isize, y: isize) -> Vec<isize> {
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

    struct TestCase(isize, isize, Vec<isize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(9, 45000, vec![0, 9, 0]),
            TestCase(20, 196000, vec![-1, -1, -1]),
            TestCase(1000, 1234000, vec![2, 54, 944]),
            TestCase(2000, 20000000, vec![2000, 0, 0]),
        ];

        for TestCase(n, y, expected) in tests {
            assert_eq!(run(n, y), expected);
        }
    }
}
