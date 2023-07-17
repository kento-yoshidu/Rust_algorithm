// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ca

#[allow(dead_code)]
pub fn run(a: usize, b: usize) -> String {
    for i in a..=b {
        if 100 % i == 0 {
            return String::from("Yes");
        }
    }

    String::from("No")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(5, 20));
        assert_eq!(String::from("No"), run(6, 9));
    }
}
