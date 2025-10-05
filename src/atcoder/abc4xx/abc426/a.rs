// https://atcoder.jp/contests/abc426/tasks/abc426_a

fn run(x: &str, y: &str) -> &'static str {
    match (x, y) {
        (a, b) if a == b => "Yes",
        ("Ocelot", _) => "No",
        ("Serval", "Ocelot") => "Yes",
        ("Serval", _) => "No",
        ("Lynx", _) => "Yes",
         _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn abc426_a() {
        let tests = [
            TestCase("Serval", "Ocelot", "Yes"),
            TestCase("Serval", "Lynx", "No"),
            TestCase("Ocelot", "Ocelot", "Yes"),
        ];

        for TestCase(x, y, expected) in tests {
            assert_eq!(run(x, y), expected);
        }
    }
}
