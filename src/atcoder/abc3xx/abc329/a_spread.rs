// https://atcoder.jp/contests/abc329/tasks/abc329_a

pub fn run(s: &str) -> String {
    s.chars()
        .map(|c| {
            format!("{} ", c)
        })
        .collect::<String>()
        .trim_end()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("A B C"), run("ABC"));
        assert_eq!(String::from("Z Z Z Z Z Z Z"), run("ZZZZZZZ"));
        assert_eq!(String::from("O O X X O O"), run("OOXXOO"));
    }
}
