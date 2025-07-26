// https://atcoder.jp/contests/abc119/tasks/abc119_b

fn run(s: &str) -> &'static str {
    let temp: Vec<_> = s.split("/").collect();

    if temp[1].parse::<i32>().unwrap() > 4 {
        "TBD"
    } else {
        "Heisei"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc119_a() {
        let tests = [
            TestCase("2019/04/30", "Heisei"),
            TestCase("2019/11/30", "TBD"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
