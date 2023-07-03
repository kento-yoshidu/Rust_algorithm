// https://atcoder.jp/contests/abc014/tasks/abc014_1

#[allow(dead_code)]
pub fn run(a: i32, b: i32) -> i32 {
    let amari = a % b;

    if amari == 0 {
        0
    } else {
        (a / b + 1) * b - a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(7, 3));
        assert_eq!(0, run(5, 5));
        assert_eq!(99, run(1, 100));
    }
}
