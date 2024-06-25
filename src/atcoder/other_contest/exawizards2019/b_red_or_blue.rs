// https://atcoder.jp/contests/exawizards2019/tasks/exawizards2019_b

fn run(n: usize, s: &str) -> &'static str {
    let count = s.chars()
        .filter(|c| *c == 'R')
        .count();

    if count > n / 2 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, "RRNR", "Yes"),
            TestCase(4, "BRBR", "No")
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
