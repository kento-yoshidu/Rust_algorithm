// https://atcoder.jp/contests/abc168/tasks/abc168_a

pub fn run(n: usize) -> String {
    match n % 10 {
        2 | 4 | 5 | 7 | 9 => String::from("hon"),
        0 | 1 | 6 | 8 => String::from("pon"),
        3 => String::from("bon"),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("pon"), run(16));
        assert_eq!(String::from("hon"), run(2));
        assert_eq!(String::from("bon"), run(183));
    }
}
