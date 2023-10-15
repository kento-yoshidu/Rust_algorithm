// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_d

pub fn run(n: usize) -> String {
    let mut ans = String::new();

    for i in (0..=9).rev() {
        let tmp = (n / (1 << i)) % 2;

        ans.push(std::char::from_digit(tmp as u32, 10).unwrap())
    }

    ans
}

pub fn run2(n: usize) -> String {
    format!("{:0>1$b}", n, 10)
}

pub fn run3(n: usize) -> String {
    (0..=9).rev().map(|i| {
        let tmp = (n / (1 << i)) % 2;
        std::char::from_digit(tmp as u32, 10).unwrap()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("0000001101"), run(13));
        assert_eq!(String::from("0000100101"), run(37));
        assert_eq!(String::from("1111101000"), run(1000));
    }

    #[test]
    fn test2() {
        assert_eq!(String::from("0000001101"), run2(13));
        assert_eq!(String::from("0000100101"), run2(37));
        assert_eq!(String::from("1111101000"), run2(1000));
    }

    #[test]
    fn test3() {
        assert_eq!(String::from("0000001101"), run3(13));
        assert_eq!(String::from("0000100101"), run3(37));
        assert_eq!(String::from("1111101000"), run3(1000));
    }
}
