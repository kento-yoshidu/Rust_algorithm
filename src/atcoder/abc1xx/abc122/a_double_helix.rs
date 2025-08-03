// https://atcoder.jp/contests/abc122/tasks/abc122_a

fn run(c: char) -> char {
    match c {
        'A' => 'T',
        'T' => 'A',
        'G' => 'C',
        _ => 'G'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(char, char);

    #[test]
    fn abc122_a() {
        let tests = [
            TestCase('A', 'T'),
            TestCase('G', 'C'),
        ];

        for TestCase(c, expected) in tests {
            assert_eq!(run(c), expected);
        }
    }
}
