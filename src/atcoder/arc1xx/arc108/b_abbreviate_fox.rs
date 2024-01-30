// https://atcoder.jp/contests/arc108/tasks/arc108_b

pub fn run(_n: usize, s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    chars.iter()
        .fold(Vec::new(), |mut stack, c| {
            stack.push(*c);

            if stack.len() >= 3 && stack[stack.len()-3..] == ['f', 'o', 'x'] {
                stack.truncate(stack.len()-3);
            }

            stack
        })
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(6, "icefox"));
        assert_eq!(7, run(7, "firebox"));
        assert_eq!(27, run(48, "ffoxoxuvgjyzmehmopfohrupffoxoxfofofoxffoxoxejffo"));
    }
}
