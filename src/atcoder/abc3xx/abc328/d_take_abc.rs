// https://atcoder.jp/contests/abc328/tasks/abc328_d

fn run(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut ans: Vec<char> = Vec::new();

    for c in chars {
        ans.push(c);
        let len = ans.len();

        if len >= 3 && &ans[len-3..] == ['A', 'B', 'C'] {
            ans.truncate(len-3);
        }
    }

    ans.iter().collect()
}

fn run2(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();

    chars.iter()
        .fold(Vec::new(), |mut state, c: &char| {
            state.push(*c);
            let len = state.len();

            if len >= 3 && &state[state.len()-3..] == ['A', 'B', 'C'] {
                state.truncate(state.len()-3);
            }

            state
        })
        .iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("BAABCBCCABCAC", "BCAC"),
            TestCase("ABCABC", ""),
            TestCase("AAABCABCABCAABCABCBBBAABCBCCCAAABCBCBCC", "AAABBBCCC"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
