// https://atcoder.jp/contests/abc151/tasks/abc151_a

pub fn run(c: char) -> char {
    (c as u8 + 1) as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!('b', run('a'));
        assert_eq!('z', run('y'));
    }
}
