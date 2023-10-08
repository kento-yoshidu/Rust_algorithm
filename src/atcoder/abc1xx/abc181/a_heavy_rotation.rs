// https://atcoder.jp/contests/abc179/tasks/abc179_a

pub fn run(n: i32) -> String {
    if n % 2 == 0 {
        String::from("White")
    } else {
        String::from("Black")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("White"), run(2));
        assert_eq!(String::from("Black"), run(5));
    }
}
