// https://atcoder.jp/contests/abc328/tasks/abc328_d

pub fn run(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();

    let mut ans: Vec<char> = Vec::new();
    ans.push('x');
    ans.push('x');

    for c in chars {
        ans.push(c);

        let len = ans.len();

        if &ans[len-3..] == ['A', 'B', 'C'] {
            ans.truncate(len-3);
        }
    }

    ans.iter().skip(2).collect()
}

pub fn run2(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();

    chars.iter()
        .fold(vec!['x', 'x'], |mut state, c: &char| {
            state.push(*c);

            if &state[state.len()-3..] == ['A', 'B', 'C'] {
                state.truncate(state.len()-3);
            }

            state
        })
        .iter()
        .skip(2)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("BCAC"), run("BAABCBCCABCAC"));
        assert_eq!(String::new(), run("ABCABC"));
        assert_eq!(String::from("AAABBBCCC"), run("AAABCABCABCAABCABCBBBAABCBCCCAAABCBCBCC"));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("BCAC"), run2("BAABCBCCABCAC"));
        assert_eq!(String::new(), run2("ABCABC"));
        assert_eq!(String::from("AAABBBCCC"), run2("AAABCABCABCAABCABCBBBAABCBCCCAAABCBCBCC"));
    }
}
