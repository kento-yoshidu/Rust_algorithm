// https://atcoder.jp/contests/abc217/tasks/abc217_a

pub fn run(s: String, t: String) -> String {
    if s < t {
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
        assert_eq!(String::from("Yes"), run(String::from("abc"), String::from("atcoder")));
        assert_eq!(String::from("No"), run(String::from("arc"), String::from("agc")));
        assert_eq!(String::from("Yes"), run(String::from("a"), String::from("aa")));
    }
}
