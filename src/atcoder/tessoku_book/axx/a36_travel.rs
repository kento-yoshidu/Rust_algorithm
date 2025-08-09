// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_aj

fn run(n: usize, k: usize) -> &'static str {
    if k >= 2*n - 2 && k % 2 == 0 {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn tessoku_a36() {
        let tests = [
            TestCase(5, 10, "Yes"),
            TestCase(5, 9, "No"),
            TestCase(5, 6, "No"),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
