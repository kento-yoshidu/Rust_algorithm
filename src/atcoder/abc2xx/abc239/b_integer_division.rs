// https://atcoder.jp/contests/abc239/tasks/abc239_b

pub fn run(x: isize) -> isize {
    if x > 0 {
        x / 10
    } else {
        if x % 10 == 0 {
            x / 10
        } else {
            x / 10 -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(47));
        assert_eq!(-3, run(-24));
        assert_eq!(5, run(50));
        assert_eq!(-3, run(-30));
        assert_eq!(98765432198765432, run(987654321987654321));
    }
}
