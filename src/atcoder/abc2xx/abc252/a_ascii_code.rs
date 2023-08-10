// https://atcoder.jp/contests/abc252/tasks/abc252_a

pub fn run(n: u8) -> char {
    n as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!('a', run(97));
        assert_eq!('z', run(122));
    }
}
