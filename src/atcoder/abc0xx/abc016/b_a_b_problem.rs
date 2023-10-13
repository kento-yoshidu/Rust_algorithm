// https://atcoder.jp/contests/abc016/tasks/abc016_2

pub fn run(a: usize, b: usize, c: usize) -> String {
    if a + b == c && a - b == c {
        String::from("?")
    } else if a + b == c {
        String::from("+")
    } else if a - b == c {
        String::from("-")
    } else {
        String::from("!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("?"), run(1, 0, 1));
        assert_eq!(String::from("+"), run(1, 1, 2));
        assert_eq!(String::from("-"), run(1, 1, 0));
        assert_eq!(String::from("!"), run(1, 1, 1));
    }
}
