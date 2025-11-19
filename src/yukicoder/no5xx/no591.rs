// https://yukicoder.me/problems/no/591

fn run(t1: char, t2: char) -> String {
    format!("({}{}{})/", t1, t2, t1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(char, char, &'static str);

    #[test]
    fn yuki_591() {
        let tests = [
            TestCase('^', 'o', "(^o^)/"),
            TestCase('-', '_', "(-_-)/"),
            TestCase('1', '+', "(1+1)/"),
        ];

        for TestCase(t1, t2, expected) in tests {
            assert_eq!(run(t1, t2), expected);
        }
    }
}
