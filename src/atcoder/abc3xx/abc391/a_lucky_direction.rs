// https://atcoder.jp/contests/abc391/tasks/abc391_a

fn run(d: &str) -> &'static str {
    match d {
        "N" => "S",
        "S" => "N",
        "W" => "E",
        "E" => "W",
        "NE" => "SW",
        "SW" => "NE",
        "NW" => "SE",
        "SE" => "NW",
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("N", "S"),
            TestCase("SE", "NW"),
        ];

        for TestCase(d, expected) in tests {
            assert_eq!(run(d), expected);
        }
    }
}
