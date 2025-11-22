// https://yukicoder.me/problems/no/969

fn run(x: usize) -> &'static str {
    match x {
        0 | 4 | 10 => "Yes",
        _ => "No",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn yuki_969() {
        let tests = [
            TestCase(0, "Yes"),
            TestCase(7, "No"),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
