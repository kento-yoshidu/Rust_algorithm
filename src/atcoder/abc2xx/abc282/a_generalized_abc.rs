// https://atcoder.jp/contests/abc282/tasks/abc282_a

fn run(k: usize) -> String {
    (0..k)
        .map(|c| (b'A' + c as u8) as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn abc282_a() {
        let tests = [
            TestCase(3, "ABC"),
            TestCase(1, "A"),
        ];

        for TestCase(k, expected) in tests {
            assert_eq!(run(k), expected);
        }
    }
}
