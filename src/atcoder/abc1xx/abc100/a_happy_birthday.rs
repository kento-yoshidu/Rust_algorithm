// https://atcoder.jp/contests/abc100/tasks/abc100_a

pub fn run(a: usize, b: usize) -> String {
    if a.max(b) <= 8 {
        String::from("Yay!")
    } else {
        String::from(":(")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yay!"), run(5, 4));
        assert_eq!(String::from("Yay!"), run(8, 8));
        assert_eq!(String::from(":("), run(11, 4));
    }
}
