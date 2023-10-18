// https://atcoder.jp/contests/abc012/tasks/abc012_2

pub fn run(n: usize) -> String {
    let h = n / (60 * 60);
    let m = n / 60 % 60;
    let s = n % 60;

    format!("{:02}:{:02}:{:02}", h, m, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("01:01:01"), run(3661));
        assert_eq!(String::from("23:59:59"), run(86399));
        assert_eq!(String::from("23:59:58"), run(86398));
    }
}
