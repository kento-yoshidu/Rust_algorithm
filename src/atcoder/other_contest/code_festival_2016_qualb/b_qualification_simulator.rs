// https://atcoder.jp/contests/code-festival-2016-qualb/tasks/codefestival_2016_qualB_b

pub fn run(_n: usize, a: usize, b: usize, s: &str) -> Vec<&'static str> {
    let vec: Vec<char> = s.chars().collect();

    let mut rest = a + b;
    let mut rest_b = b;

    let mut ans = Vec::new();

    for c in vec {
        if c == 'c' {
            ans.push("No");
            continue;
        }

        if c == 'a' {
            if rest == 0 {
                ans.push("No");
            } else {
                ans.push("Yes");
                rest -= 1;
            }
        }

        if c == 'b' {
            if rest == 0 || rest_b == 0 {
                ans.push("No");
            } else {
                ans.push("Yes");
                rest_b -= 1;
                rest -= 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, &'static str, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, 2, 3, "abccabaabb", vec!["Yes", "Yes", "No", "No", "Yes", "Yes", "Yes", "No", "No", "No"]),
            TestCase(12, 5, 2, "cabbabaacaba", vec!["No", "Yes", "Yes", "Yes", "Yes", "No", "Yes", "Yes", "No", "Yes", "No", "No"]),
            TestCase(5, 2, 2, "ccccc", vec!["No", "No", "No", "No", "No"]),
        ];

        for TestCase(n, a, b, s, expected) in tests {
            assert_eq!(run(n, a, b, s), expected);
        }
    }
}
