// https://atcoder.jp/contests/code-festival-2016-qualb/tasks/codefestival_2016_qualB_a

fn run(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    let vec = ['C', 'O', 'D', 'E', 'F', 'E', 'S', 'T', 'I', 'V', 'A', 'L', '2', '0', '1', '6'];

    chars.iter()
        .zip(vec.iter())
        .filter(|(a, b)| {
            a != b
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("C0DEFESTIVAL2O16", 2),
            TestCase("FESTIVAL2016CODE", 16),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
