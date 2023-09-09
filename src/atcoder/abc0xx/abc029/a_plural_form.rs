// https://atcoder.jp/contests/abc029/tasks/abc029_a

pub fn run(w: String) -> String {
    w + "s"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("dogs"), run(String::from("dog")));
        assert_eq!(String::from("chokudais"), run(String::from("chokudai")));
    }
}
