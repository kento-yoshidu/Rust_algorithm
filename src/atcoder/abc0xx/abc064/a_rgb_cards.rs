// https://atcoder.jp/contests/abc064/tasks/abc064_a

fn run(r: usize, g: usize, b: usize) -> &'static str {
    let num = r * 100 + g * 10 + b;

    if num % 4 == 0 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 3, 2, "Yes"),
            TestCase(2, 3, 4, "No"),
        ];

        for TestCase(r, g, b, expected) in tests {
            assert_eq!(run(r, g, b), expected);
        }
    }
}
