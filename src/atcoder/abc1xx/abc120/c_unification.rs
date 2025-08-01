// https://atcoder.jp/contests/abc120/tasks/abc120_c

fn run(s: &str) -> usize {
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

    struct TestCase(&'static str, usize);

    #[test]
    fn abc120_c() {
        let tests = [
            TestCase("0011", 4),
            TestCase("11011010001011", 12),
            TestCase("0", 0),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
