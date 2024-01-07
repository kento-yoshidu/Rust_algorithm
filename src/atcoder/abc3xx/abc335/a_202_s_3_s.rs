// https://atcoder.jp/contests/abc335/tasks/abc335_a

pub fn run(s: &str) -> String {
    format!("{}4", &s[0..s.len()-1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("hello2024"), run("hello2023"));
        assert_eq!(String::from("worldtourfinals2024"), run("worldtourfinals2023"));
        assert_eq!(String::from("2024"), run("2023"));
        assert_eq!(String::from("20232024"), run("20232023"));
    }
}
