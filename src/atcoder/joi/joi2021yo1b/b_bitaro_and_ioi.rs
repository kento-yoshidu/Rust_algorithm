// https://atcoder.jp/contests/joi2021yo1b/tasks/joi2021_yo1b_b

fn run(n: usize, s: &str) -> &'static str {
    let chars: Vec<char> = s.chars().collect();

    if n < 3 {
        return "No";
    }

    for i in 0..n {
        if chars[i] != 'I' {
            continue;
        }

        for j in i+1..n {
            if chars[j] != 'O' {
                continue;
            }

            for k in j+1..n {
                if chars[k] == 'I' {
                    return "Yes";
                }
            }
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, "BITAROOI", "Yes"),
            TestCase(6, "BBOOII", "No"),
            TestCase(5, "IOIOI", "Yes"),
            TestCase(9, "RATRATRAT", "No"),
            TestCase(1, "A", "No"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
