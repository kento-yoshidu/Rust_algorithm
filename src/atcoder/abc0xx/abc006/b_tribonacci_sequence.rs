// https://atcoder.jp/contests/abc006/tasks/abc006_2

// https://atcoder.jp/contests/abc006/tasks/abc006_2

pub fn run(n: usize) -> usize {
    (0..n-1)
        .fold((0, 0, 1), |state, _| {
            let sum = state.0 + state.1 + state.2;

            (state.1 % 10007, state.2 % 10007, sum % 10007)
        }).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(7, run(7));
        assert_eq!(0, run(2));
        assert_eq!(7927, run(100000));
    }
}
