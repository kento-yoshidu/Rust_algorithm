// https://atcoder.jp/contests/agc005/tasks/agc005_a

pub fn run(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    chars.iter()
        .skip(1)
        .fold(vec![chars[0]], |mut state, c| {
            state.push(*c);

            let len = state.len();

            if len <= 1 {
                state
            } else if state[len-2] == 'S' && state[len-1] == 'T' {
                state.truncate(len-2);
                state
            } else {
                state
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
