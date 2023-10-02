// https://atcoder.jp/contests/abc183/tasks/abc183_a

pub fn run(x: i8) -> i8 {
    if x < 0 {
        0
    } else {
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(1));
        assert_eq!(0, run(0));
        assert_eq!(0, run(-1));
    }
}
