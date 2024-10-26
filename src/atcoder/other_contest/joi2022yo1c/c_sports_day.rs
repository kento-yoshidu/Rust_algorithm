// https://atcoder.jp/contests/joi2022yo1c/tasks/joi2022_yo1c_c

fn run(_n: usize, k: usize, s: &str) -> char {
    let r = s.chars()
        .filter(|c| {
            *c == 'R'
        })
        .count();

    if k == r+1 {
        'R'
    } else {
        'W'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, char);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 3, "RWWRWW", 'R'),
            TestCase(5, 3, "RWRR", 'W'),
            TestCase(70, 1, "WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW", 'R'),
        ];

        for TestCase(n, k, s, expected) in tests {
            assert_eq!(run(n, k, s), expected);
        }
    }
}
