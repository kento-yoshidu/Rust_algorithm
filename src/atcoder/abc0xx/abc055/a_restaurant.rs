// https://atcoder.jp/contests/abc055/tasks/abc055_a

#[allow(dead_code)]
pub fn run(n: i32) -> i32 {
    let pay = n * 800;
    let back = (n / 15) * 200;

    pay - back
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(15800, run(20));
        assert_eq!(47200, run(60));
    }
}
