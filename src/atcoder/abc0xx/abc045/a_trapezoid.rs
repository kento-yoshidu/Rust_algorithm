// https://atcoder.jp/contests/abc045/tasks/abc045_a

pub fn run(a: i32, b: i32, h: i32) -> i32 {
    (a + b) * h / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(7, run(3, 4, 2));
        assert_eq!(16, run(4, 4, 4));
    }
}
