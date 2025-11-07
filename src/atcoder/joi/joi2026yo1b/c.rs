// https://atcoder.jp/contests/joi2026yo1b/tasks/joi2026_yo1b_c

fn run(_n: usize, s: &str) -> usize {
    s.chars()
        .collect::<Vec<char>>()
        .windows(3)
        .filter(|str| str == &&['A', 'O', 'I'] || str == &&['I', 'O', 'I'])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize);

    #[test]
    fn joi2026yo1b_c() {
        let tests = [
            TestCase(5, "IAOIO", 1),
            TestCase(8, "AOIOIOIA", 3),
            TestCase(6, "IIOOII", 0),
            TestCase(15, "IAOIAOAOIOIAIOI", 4),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
