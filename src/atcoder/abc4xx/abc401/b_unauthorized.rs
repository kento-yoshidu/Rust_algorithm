// https://atcoder.jp/contests/abc401/tasks/abc401_b

fn run(_n: usize, s: Vec<&str>) -> usize {
    let mut ans = 0;

    let mut is_login = false;

    for str in s {
        match str {
            "login" => {
                if !is_login {
                    is_login = true;
                }
            },
            "logout" => {
                if is_login {
                    is_login = false;
                }
            },
            "private" => {
                if !is_login {
                    ans += 1;
                }
            },
            _ => (),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, vec!["login", "private", "public", "logout", "private", "public"], 1),
            TestCase(4, vec!["private", "private", "private", "logout"], 3),
            TestCase(20, vec![ "private", "login", "private", "logout", "public", "logout", "logout", "logout", "logout", "private", "login", "login", "private", "login", "private", "login", "public", "private", "logout", "private"], 3),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
