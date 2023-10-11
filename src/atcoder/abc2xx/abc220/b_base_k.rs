// https://atcoder.jp/contests/abc220/tasks/abc220_b

pub fn run(k: u32, a: String, b: String) -> usize {
    usize::from_str_radix(&a, k).unwrap() * usize::from_str_radix(&b, k).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(220, run(2, String::from("1011"), String::from("10100")));
        assert_eq!(15642, run(7, String::from("123"), String::from("456")));
    }
}
