// https://atcoder.jp/contests/agc005/tasks/agc005_a

pub fn run(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    chars.iter()
        .fold(Vec::new(), |mut stack, c| {
            stack.push(*c);

            let len = stack.len();

            if len >= 2 && stack[len-2] == 'S' && stack[len-1] == 'T' {
                stack.truncate(len-2);
                stack
            } else {
                stack
            }
        })
        .iter()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run("TSTTSS"));
        assert_eq!(0, run("SSTTST"));
        assert_eq!(4, run("TSSTTTSS"));
    }
}
