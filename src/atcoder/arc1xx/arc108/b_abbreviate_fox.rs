// https://atcoder.jp/contests/arc108/tasks/arc108_b

pub fn run(_n: usize, s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    chars.iter()
        .fold(vec!['x', 'x'], |mut state, c| {
            state.push(*c);

            if state[state.len()-3..] == ['f', 'o', 'x'] {
                state.truncate(state.len()-3);
            }

            state
        })
        .len() - 2
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
