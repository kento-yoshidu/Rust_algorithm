// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_b

fn run(_n: usize, x: usize, vec: &Vec<usize>) -> &'static str {
    for i in vec.into_iter() {
        if *i == x {
            return  "Yes";
        }
    }

    "No"
}

fn run2(_n: usize, x: usize, vec: &Vec<usize>) -> &'static str {
    if vec.into_iter().any(|num| {
        *num == x
    }) {
        "Yes"
    } else {
        "No"
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, &'static str);

    #[test]
    fn tessoku_a02() {
        let tests = [
            TestCase(5, 40, vec![10, 20, 30, 40, 50], "Yes"),
        ];

        for TestCase(n, x, vec, expected) in tests {
            assert_eq!(run(n, x, &vec), expected);
            assert_eq!(run2(n, x, &vec), expected);
        }
    }
}
