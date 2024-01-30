// https://atcoder.jp/contests/abc286/tasks/abc286_b

pub fn run(n: usize, s: &str) -> String {
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

    ans.iter().collect()
}

pub fn run2(_n: usize, s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();

    chars.iter()
        .fold(Vec::new(), |mut stack, c| {
            stack.push(*c);

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

    #[test]
    fn test() {
        assert_eq!(String::from("nyaan"), run(4, "naan"));
        assert_eq!(String::from("near"), run(4, "near"));
        assert_eq!(String::from("nyationyal"), run(8, "national"));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("nyaan"), run2(4, "naan"));
        assert_eq!(String::from("near"), run2(4, "near"));
        assert_eq!(String::from("nyationyal"), run2(8, "national"));
    }
}
