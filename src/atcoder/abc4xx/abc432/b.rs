// https://atcoder.jp/contests/abc432/tasks/abc432_b

use itertools::Itertools;

fn run(x: &str) -> usize {
    let mut chars: Vec<char> = x.chars().sorted().collect();

    if chars[0] == '0' {
        let pos = chars.iter().position(|c| *c != '0').unwrap();

        chars.swap(0, pos);
    }

    chars.into_iter().collect::<String>().parse::<usize>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc432_b() {
        let tests = [
            TestCase("903", 309),
            TestCase("432", 234),
            TestCase("100", 100),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
