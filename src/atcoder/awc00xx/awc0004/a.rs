// https://atcoder.jp/contests/awc0004/tasks/awc0004_a

fn run(_n: usize, s: usize, t: usize, a: Vec<usize>) -> &'static str {
    if a.into_iter().sum::<usize>() <= (t - s) * 60 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, &'static str);

    #[test]
    fn awc0004_a() {
        let tests = [
            TestCase(3, 9, 10, vec![15, 20, 10], "Yes"),
            TestCase(5, 14, 17, vec![45, 30, 25, 40, 50], "No"),
            TestCase(10, 6, 12, vec![30, 25, 45, 15, 20, 35, 40, 10, 50, 60], "Yes"),
        ];

        for TestCase(n, s, t, a, expected) in tests {
            assert_eq!(run(n, s, t, a), expected);
        }
    }
}
