// https://atcoder.jp/contests/abc453/tasks/abc453_a

fn run(_n: usize, s: &str) -> String {
    let mut ans = String::new();

    let mut flag = false;

    for c in s.chars() {
        if c == 'o' {
            if flag {
                ans.push(c);
            }
        } else {
            if !flag {
                flag = true;
            }

            ans.push(c);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn abc453_a() {
        let tests = [
            TestCase(7, "ooparts", "parts"),
            TestCase(6, "abcooo", "abcooo"),
            TestCase(5, "ooooo", ""),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
