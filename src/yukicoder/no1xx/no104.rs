// https://yukicoder.me/problems/no/104

fn run(s: &str) -> usize {
    if s.len() == 0 {
        return 1;
    }

    s.chars()
        .fold(1, |acc, c| {
            match c {
                'L' => acc * 2,
                'R' => acc * 2 + 1,
                _ => unreachable!()
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn yuki_104() {
        let tests = [
            TestCase("LR", 5),
            TestCase("RLL", 12),
            TestCase("RLLRLRLRRRLRL", 12986),
            TestCase("", 1),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
