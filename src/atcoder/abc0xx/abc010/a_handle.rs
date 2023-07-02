// https://atcoder.jp/contests/abc010/tasks/abc010_1

#[allow(dead_code)]
pub fn run(s: String) -> String {
    s + "pp"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("chokudaipp"), run(String::from("chokudai")));
    }
}
