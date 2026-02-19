// https://atcoder.jp/contests/abc252/tasks/abc252_a

fn run(n: u8) -> char {
    n as char
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(u8, char);

    #[test]
    fn abc252_a() {
        let tests = [
            TestCase(97, 'a'),
            TestCase(122, 'z'),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
