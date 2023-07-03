// https://atcoder.jp/contests/abc013/tasks/abc013_1

#[allow(dead_code)]
pub fn run(x: char) -> u8 {
    (x as u8 - 'A' as u8) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run('A'));
        assert_eq!(2, run('B'));
        assert_eq!(3, run('C'));
        assert_eq!(4, run('D'));
    }
}
