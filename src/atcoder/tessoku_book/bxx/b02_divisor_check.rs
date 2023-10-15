// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ca

pub fn run(a: usize, b: usize) -> String {
    for i in a..=b {
        if 100 % i == 0 {
            return String::from("Yes");
        }
    }

    String::from("No")
}

pub fn run2(a: usize, b: usize) -> String {
    if (a..=b).any(|i| {
        100 % i == 0
    }) {
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
        assert_eq!(String::from("Yes"), run(5, 20));
        assert_eq!(String::from("No"), run(6, 9));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("Yes"), run2(5, 20));
        assert_eq!(String::from("No"), run2(6, 9));
    }
}
