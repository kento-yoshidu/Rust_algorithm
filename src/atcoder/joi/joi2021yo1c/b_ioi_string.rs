// https://atcoder.jp/contests/joi2021yo1c/tasks/joi2021_yo1c_b

fn run(_n: usize, s: &str) -> usize {
    s.chars()
        .enumerate()
        .filter(|(i, c)| {
            match i % 2 {
                0 => *c != 'I',
                1 => *c != 'O',
                _ => unreachable!(),
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, "JJOOI", 3),
            TestCase(7, "IOIOIOI", 0),
            TestCase(7, "BEAVERS", 7),
            TestCase(5, "OIOIO", 5),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
