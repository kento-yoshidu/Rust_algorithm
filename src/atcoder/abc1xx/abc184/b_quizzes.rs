// https://atcoder.jp/contests/abc184/tasks/abc184_b

pub fn run(_n: usize, x: usize, s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    chars.iter()
        .fold(x, |state, char| {
            if *char == 'o' {
                state + 1
            } else {
                if state == 0 {
                    0
                } else {
                    state - 1
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, run(3, 0, "xox"));
        assert_eq!(200017, run(20, 199999, "oooooooooxoooooooooo"));
        assert_eq!(0, run(20, 10, "xxxxxxxxxxxxxxxxxxxx"));
    }
}
