// https://atcoder.jp/contests/abc286/tasks/abc286_b

fn run(n: usize, s: &str) -> String {
    let mut ans = Vec::new();
    let chars: Vec<char> = s.chars().collect();

    for i in 0..n {
        ans.push(chars[i]);
        let len = ans.len();

        if len >= 2 {
            if ans[len-2..] == ['n', 'a'] {
                ans.truncate(len-2);
                ans.append(&mut vec!['n', 'y', 'a']);
            }
        }
    }

    ans.into_iter().collect()
}

fn run2(_n: usize, s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();

    chars.into_iter()
        .fold(Vec::new(), |mut stack, c| {
            stack.push(c);

            if stack.len() >= 2 && stack[stack.len()-2..] == ['n', 'a'] {
                stack.truncate(stack.len()-2);
                stack.append(&mut vec!['n', 'y', 'a']);
            }

            stack
        })
        .iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn abc286_b() {
        let tests = [
            TestCase(4, "naan", "nyaan"),
            TestCase(4, "near", "near"),
            TestCase(8, "national", "nyationyal"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
            assert_eq!(run2(n, s), expected);
        }
    }
}
