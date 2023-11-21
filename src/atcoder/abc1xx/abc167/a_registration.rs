// https://atcoder.jp/contests/abc167/tasks/abc167_a

pub fn run(s: &str, t: &str) -> String {
    if s == &t[0..t.len()-1] {
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
        assert_eq!(String::from("Yes"), run("chokudai", "chokudaiz"));
        assert_eq!(String::from("No"), run("snuke", "snekee"));
        assert_eq!(String::from("Yes"), run("a", "aa"));
    }
}