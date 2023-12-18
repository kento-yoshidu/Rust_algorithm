// https://atcoder.jp/contests/abc120/tasks/abc120_c

pub fn run(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    chars.iter()
        .skip(1)
        .fold((0, vec![chars[0]]), |(ans, mut state), c| {
            state.push(*c);

            if state.len() <= 1 {
                (ans, state)
            } else if state[state.len()-2] != state[state.len()-1] {
                state.truncate(state.len()-2);
                (ans+2, state)
            } else {
                (ans, state)
            }
        }).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run("0011"));
        assert_eq!(12, run("11011010001011"));
        assert_eq!(0, run("0"));
    }
}
