// https://atcoder.jp/contests/arc047/tasks/arc047_a

pub fn run(_n: usize, l: usize, s: &str) -> usize {
    s.chars()
        .fold((0, 1), |(ans, state), c| {
            if c == '+' {
                if state + 1 > l {
                    (ans + 1, 1)
                } else {
                    (ans, state + 1)
                }
            } else {
                (ans, state - 1)
            }
        }).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(6, 2, "+++-++"));
        assert_eq!(0, run(20, 20, "++-+-+++--+++++-++++"));
    }
}
