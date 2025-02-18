// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_di

fn run(_n: usize, k: usize, s: &str) -> &'static str {
    let count = s.chars()
        .filter(|c| *c == '1')
        .count() as isize;

    if (k as isize - count) % 2 == 0 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, 3, "1010111", "Yes"),
            TestCase(10, 6, "0001010001", "No"),
            TestCase(2, 2, "11", "Yes"),
        ];

        for TestCase(n, k, s, expected) in tests {
            assert_eq!(run(n, k, s), expected);
        }
    }
}
