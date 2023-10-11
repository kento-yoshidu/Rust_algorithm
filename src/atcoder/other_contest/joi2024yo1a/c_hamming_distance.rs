// https://atcoder.jp/contests/joi2024yo1a/tasks/joi2024_yo1a_c

pub fn run(_n: usize, s: &str, t: &str) -> usize {
    s.chars()
        .zip(t.chars())
        .filter(|v| { v.0 != v.1 })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(3, "joi", "ioi"));
        assert_eq!(4, run(5, "march", "april"));
        assert_eq!(0, run(6, "sample", "sample"));
    }
}
