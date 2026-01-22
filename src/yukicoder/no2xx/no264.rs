// https://yukicoder.me/problems/no/264

fn run(n: usize, k: usize) -> &'static str {
    match (n, k) {
        (a, b) if a == b => "Drew",
        (0, 1) => "Win",
        (0, 2) => "Lost",
        (1, 0) => "Lost",
        (1, 2) => "Win",
        (2, 0) => "Win",
        (2, 1) => "Lost",
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn yuki_264() {
        let tests = [
            TestCase(0, 0, "Drew"),
            TestCase(2, 0, "Win"),
            TestCase(2, 1, "Lost"),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
