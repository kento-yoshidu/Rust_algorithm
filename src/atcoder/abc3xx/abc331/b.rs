// https://atcoder.jp/contests/abc331/tasks/abc331_b

use itertools::iproduct;

fn run(n: usize, s: usize, m: usize, l: usize) -> usize {
    let mut ans = std::usize::MAX;

    for i in 0..=100 {
        for j in 0..=100 {
            for k in 0..=100 {
                if i*6 + j*8 + k*12 >= n {
                    ans = ans.min(i*s + j*m + k*l);
                }
            }

            if i*6 + j*8 > n {
                break;
            }
        }

        if i*6 > n {
            break
        }
    }

    ans
}

fn run2(n: usize, s: usize, m: usize, l: usize) -> usize {
    let mut ans = std::usize::MAX;

    for (i, j, k) in iproduct!(0..=100, 0..=100, 0..=100) {
        if i*6 + j*8 + k*12 >= n {
            ans = ans.min(i*s + j*m + k*l);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize, usize);

    #[test]
    fn abc331_b() {
        let tests = [
            TestCase(16, 120,  150, 200, 300),
            TestCase(10, 100, 50, 10, 10),
            TestCase(99, 600, 800, 1200, 10000),
        ];

        for TestCase(n, s, m, l, expected) in tests {
            assert_eq!(run(n, s, m, l), expected);
            assert_eq!(run2(n, s, m, l), expected);
        }
    }
}
