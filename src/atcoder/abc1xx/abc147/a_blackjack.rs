// https://atcoder.jp/contests/abc147/tasks/abc147_a

pub fn run(a: usize, b: usize, c: usize) -> String {
    if a + b + c >= 22 {
        String::from("bust")
    } else {
        String::from("win")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("win"), run(5, 7, 9));
        assert_eq!(String::from("bust"), run(13, 7, 2));
    }
}