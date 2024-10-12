// https://atcoder.jp/contests/abc375/tasks/abc375_a

fn run(_n: usize, s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    chars.windows(3)
        .filter(|a| {
            a[0] == '#' && a[1] == '.' && a[2] == '#'
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
            TestCase(6, "#.##.#", 2),
            TestCase(1, "#", 0),
            TestCase(9, "##.#.#.##", 3),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
