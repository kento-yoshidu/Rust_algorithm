// https://atcoder.jp/contests/abc237/tasks/abc237_a

pub fn run(n: isize) -> String {
    if -2_isize.pow(31) <= n && n < 2_isize.pow(31) {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(5));
        assert_eq!(String::from("No"), run(-9876543210));
        assert_eq!(String::from("No"), run(483597848400000));
    }
}
