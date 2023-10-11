// https://atcoder.jp/contests/abc007/tasks/abc007_2

fn run(a: String) -> String {
    if a == "a" {
        String::from("-1")
    } else {
        String::from("a")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("a"), run(String::from("xyz")));
        assert_eq!(String::from("a"), run(String::from("c")));
        assert_eq!(String::from("-1"), run(String::from("a")));
        assert_eq!(String::from("a"), run(String::from("aaaaa")));
    }
}
