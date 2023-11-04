// https://atcoder.jp/contests/abc192/tasks/abc192_b

pub fn run(s: &str) -> String {
    if s.chars()
        .enumerate()
        .all(|(i, c)| {
            if i % 2 == 0 {
                c.is_lowercase()
            } else {
                c.is_uppercase()
            }
        }) {
            String::from("Yes")
        } else {
            String::from("No")
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run("dIfFiCuLt"));
        assert_eq!(String::from("No"), run("eASY"));
        assert_eq!(String::from("Yes"), run("a"));
        assert_eq!(String::from("No"), run("A"));
    }
}
