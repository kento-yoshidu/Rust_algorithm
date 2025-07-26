// https://atcoder.jp/contests/abc122/tasks/abc122_b

fn run(str: &str) -> usize {
    let mut len = 0;
    let mut max_len = 0;

    for c in str.chars() {
        if "ACGT".contains(c) {
            len += 1;
            max_len = max_len.max(len);
        } else {
            len = 0;
        }
    }

    max_len
}

fn run2(str: &str) -> usize {
    let chars: Vec<char> = str.chars().collect();

    chars.iter()
        .fold((0, 0), |(ans, state), c| {
            if "ACGT".contains(*c) {
                (ans.max(state+1), state+1)
            } else {
                (ans, 0)
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc122_b() {
        let tests = [
            TestCase("ATCODER", 3),
            TestCase("HATAGAYA", 5),
            TestCase("SHINJUKU", 0),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
