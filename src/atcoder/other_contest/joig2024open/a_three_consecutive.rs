// https://atcoder.jp/contests/joig2024-open/tasks/joig2024_a

fn run(_n: usize, s: &str) -> &'static str {
    let vec: Vec<char> = s.chars().collect();

    if vec.windows(3)
        .any(|arr| {
            arr[0] == 'o' && arr[1] == 'o' && arr[2] == 'o'
        }) {
            "Yes"
        } else {
            "No"
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, "oxooo", "Yes"),
            TestCase(5, "xooxo", "No"),
            TestCase(1, "o", "No"),
            TestCase(10, "oooooooooo", "Yes"),
            TestCase(20, "xooxxoooxoxooxooxoox", "Yes"),
            TestCase(20, "xooxxxooxoxooxooxoox", "No")
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
