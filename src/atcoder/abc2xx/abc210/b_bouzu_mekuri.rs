// https://atcoder.jp/contests/abc210/tasks/abc210_b

pub fn run(_n: usize, s: &str) -> String {
    s.chars()
        .enumerate()
        .find(|(_, c)| {
            *c == '1'
        })
        .map(|(i, _)| {
            if i % 2 == 0 {
                String::from("Takahashi")
            } else {
                String::from("Aoki")
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Takahashi"), run(5, "00101"));
        assert_eq!(String::from("Aoki"), run(3, "010"));
    }
}
