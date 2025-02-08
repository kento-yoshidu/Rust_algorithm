// https://atcoder.jp/contests/abc097/tasks/abc097_b

use std::cmp::max;

fn run(x: usize) -> usize {
    let mut ans = 1;

    for i in 2..x {
        let mut j = i * i;

        while j <= x {
            ans = max(ans, j);
            j *= i;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 9),
            TestCase(1, 1),
            TestCase(999, 961),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
