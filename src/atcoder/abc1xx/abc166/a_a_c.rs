// https://atcoder.jp/contests/abc166/tasks/abc166_a

pub fn run(s: &str) -> String {
    match s {
        "ABC" => String::from("ARC"),
        "ARC" => String::from("ABC"),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("ARC"), run("ABC"));
        assert_eq!(String::from("ABC"), run("ARC"));
    }
}
