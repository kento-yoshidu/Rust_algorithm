// https://atcoder.jp/contests/abc469/tasks/abc469_b

fn run(n: usize, s: &str) -> usize {
    if n == 1 {
        if s == "o" {
            return 0;
        } else {
            return 1;
        }
    }

    let s: Vec<char> = s.chars().collect();

    let mut ans = 0;

    for (i, c) in s.iter().enumerate() {
        if *c == 'o' {
            continue;
        }

        if i == 0 {
            if s[i+1] == 'x' {
                ans += 1;
            }
        } else if i == n - 1 {
            if s[n-2] == 'x' {
                ans += 1;
            }
        } else {
            if s[i-1] == 'x' && s[i+1] == 'x' {
                ans += 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize);

    #[test]
    fn abc469_b() {
        let tests = [
            TestCase(8, "xxoxxxox", 2),
            TestCase(5, "ooooo", 0),
            TestCase(1, "x", 1),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
