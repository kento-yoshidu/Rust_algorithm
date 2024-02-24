// https://atcoder.jp/contests/arc003/tasks/arc003_1

fn run(n: usize, r: &str) -> f64 {
    let chars: Vec<char> = r.chars().collect();

    let sum = chars.iter()
        .fold(0, |state, c| {
            match c {
                'A' => state + 4,
                'B' => state + 3,
                'C' => state + 2,
                'D' => state + 1,
                'F' => state,
                _ => unreachable!(),
            }
        });

    sum as f64 / n as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, f64);

    #[test]
    fn test() {
        let tests = [
            TestCase(34, "ABABAAABACDDDABADFFABABDABFAAABFAA", 2.7941176470588234),
            TestCase(5, "FFFFF", 0.0),
        ];

        for TestCase(n, r, expected) in tests {
            assert_eq!(run(n, r), expected);
        }
    }
}