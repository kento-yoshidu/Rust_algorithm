// https://atcoder.jp/contests/abc362/tasks/abc362_a

use std::cmp::min;

fn run(r: usize, g: usize, b: usize, c: &str) -> usize {
    match c {
        "Blue" => {
            min(r, g)
        },
        "Green" => {
            min(r, b)
        },
        "Red" => {
            min(g, b)
        },
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(20, 30, 10, "Blue", 20),
            TestCase(100, 100, 100, "Red", 100),
            TestCase(37, 39, 93, "Blue", 37),
        ];

        for TestCase(r, g, b, c, expected) in tests {
            assert_eq!(run(r, g, b, c), expected);
        }
    }
}
