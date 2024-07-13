// https://atcoder.jp/contests/abc216/tasks/abc216_a

fn run(s: &str) -> String {
    let vec: Vec<usize> = s.split('.').map(|c| c.parse::<usize>().unwrap()).collect();

    match vec[1] {
        0..=2 => {
            format!("{}-", vec[0])
        },
        3..=6 => {
            vec[0].to_string()
        },
        7..=9 => {
            format!("{}+", vec[0])
        },
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("15.8", "15+"),
            TestCase("1.0", "1-"),
            TestCase("12.5", "12"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
