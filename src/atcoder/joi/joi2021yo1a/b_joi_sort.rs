// https://atcoder.jp/contests/joi2021yo1a/tasks/joi2021_yo1a_b

use itertools::Itertools;

fn run(_n: usize, s: &str) -> String {
    s.chars()
        .sorted_by_key(|c| {
            match *c {
                'J' => 0,
                'O' => 1,
                'I' => 2,
                _ => unreachable!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, "JIOIJO", "JJOOII"),
            TestCase(4, "OOOI", "OOOI"),
            TestCase(10, "OIJJJIOIOI", "JJJOOOIIII"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
