// https://atcoder.jp/contests/abc381/tasks/abc381_a

fn run(n: usize, s: &str) -> &'static str {
    if n % 2 == 0 {
        return "No";
    }

    if s.chars().filter(|c| *c == '/').count() != 1 {
        return "No";
    }

    let mut flag = true;

    let mut count = (0, 0);

    for c in s.chars() {
        if c == '/' {
            flag = false;
        }
        if flag {
            if c == '1' {
                count.0 += 1;
            }
        } else {
            if c == '2' {
                count.1 += 1;
            }
        }
    }

    if count.0 == n/2 && count.1 == n/2 {
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
            TestCase(5, "11/22", "Yes"),
            TestCase(4, "1/22", "No"),
            TestCase(5, "22/11", "No"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
