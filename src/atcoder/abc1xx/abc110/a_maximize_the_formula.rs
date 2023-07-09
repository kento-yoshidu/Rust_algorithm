// https://atcoder.jp/contests/abc110/tasks/abc110_a

#[allow(dead_code)]
pub fn run(a: i32, b: i32, c: i32) -> i32 {
    let mut vec = vec![a, b, c];

    vec.sort();

    vec[2] * 10 + vec[0] + vec[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(53, run(1, 5, 2));
        assert_eq!(108, run(9, 9, 9));
        assert_eq!(82, run(6, 6, 7));
    }
}
