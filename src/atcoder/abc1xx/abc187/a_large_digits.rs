// https://atcoder.jp/contests/abc187/tasks/abc187_a

use std::cmp::max;

fn run(a: usize, b: usize) -> usize {
    let total_a = (a / 100) + (a % 100 / 10) + (a % 10);
    let total_b = (b / 100) + (b % 100 / 10) + (b % 10);

    max(total_a, total_b)
}

fn calc(n: usize) -> usize {
    n.to_string()
        .chars()
        .map(|i| i.to_digit(10).unwrap() as usize)
        .sum()
}

fn run2(a: usize, b: usize) -> usize {
    let total_a = calc(a);
    let total_b = calc(b);

    max(total_a, total_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn abc187_a() {
        let tests = [
            TestCase(123, 234 , 9),
            TestCase(593, 953, 17),
            TestCase(100, 999, 27),
        ];

        for TestCase(a, b, expected) in tests {
            assert_eq!(run(a, b), expected);
        }
    }
}
