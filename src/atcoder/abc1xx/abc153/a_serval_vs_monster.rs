// https://atcoder.jp/contests/abc153/tasks/abc153_a

pub fn run(hp: i32, a: i32) -> i32 {
    if hp % a == 0 {
        hp / a
    } else {
        hp / a + 1
    }
}

pub fn run2(hp: usize, a: usize) -> usize {
    (hp as f64 / a as f64).ceil() as usize
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(10, 4));
        assert_eq!(10000, run(10000, 1));
        assert_eq!(1, run(1, 10000));
    }

    #[test]
    fn test2() {
        assert_eq!(3, run2(10, 4));
        assert_eq!(10000, run2(10000, 1));
        assert_eq!(1, run2(1, 10000));
    }
}
