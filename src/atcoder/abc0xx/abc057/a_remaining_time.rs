// https://atcoder.jp/contests/abc057/tasks/abc057_a

#[allow(dead_code)]
pub fn run(a: i32, b: i32) -> i32 {
    let time = a + b;

    if time >= 24 {
        time - 24
    } else {
        time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(21, run(9, 12));
        assert_eq!(19, run(19, 0));
        assert_eq!(1, run(23, 2));
    }
}
