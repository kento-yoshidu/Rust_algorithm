// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_d

#[allow(dead_code)]
pub fn run(n: usize) -> String {
    let mut ans = String::new();

    for i in (0..=9).rev() {
        let tmp = (n / (1 << i)) % 2;

        ans.push(std::char::from_digit(tmp as u32, 10).unwrap())
    }

    ans
}

#[allow(dead_code)]
pub fn run2(n: usize) -> String {
    format!("{:0>1$b}", n, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("0000001101"), run2(13));
        assert_eq!(String::from("0000100101"), run2(37));
        assert_eq!(String::from("1111101000"), run2(1000));
    }
}
