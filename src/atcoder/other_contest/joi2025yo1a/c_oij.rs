// https://atcoder.jp/contests/joi2025yo1a/tasks/

fn run(_n: usize, s: &str) -> String {
    s.chars()
        .map(|c| {
            match c {
                'J' => 'O',
                'O' => 'I',
                'I' => 'J',
                _ => unreachable!()
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
            TestCase(3, "JOI", "OIJ"),
            TestCase(10, "JOIOOJOOOJ", "OIJIIOIIIO"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }



}