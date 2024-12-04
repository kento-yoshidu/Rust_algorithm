// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_d

fn run(n: usize) -> String {
    let mut ans = String::new();

    for i in (0..=9).rev() {
        let tmp = (n / (1 << i)) % 2;

        ans.push(std::char::from_digit(tmp as u32, 10).unwrap())
    }

    ans
}

fn run2(n: usize) -> String {
    format!("{:0>1$b}", n, 10)
}

fn run3(n: usize) -> String {
    (0..=9).rev().map(|i| {
        let tmp = (n / (1 << i)) % 2;
        std::char::from_digit(tmp as u32, 10).unwrap()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(13, "0000001101"),
            TestCase(37, "0000100101"),
            TestCase(1000, "1111101000"),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
