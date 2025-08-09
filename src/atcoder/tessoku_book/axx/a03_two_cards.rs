// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_c

fn run(_n: usize, k: usize, pp: &Vec<usize>, qq: &Vec<usize>) -> &'static str {
    for p in pp.iter() {
        for q in qq.iter() {
            if k == (p + q) {
                return "Yes";
            }
        }
    }

    "No"
}

fn run2(_n: usize, k: usize, pp: &Vec<usize>, qq: &Vec<usize>) -> &'static str {
    if pp.iter().any(|p| {
        qq.iter().any(|q| {
            *p + *q == k
        })
    }) {
        "Yes"
    } else {
        "No"
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, &'static str);

    #[test]
    fn tessoku_a03() {
        let tests = [
            TestCase(3, 100, vec![17, 57, 99], vec![10, 36, 53], "No"),
            TestCase(5, 53, vec![10, 20, 30, 40, 50], vec![1, 2, 3, 4, 5], "Yes"),
        ];

        for TestCase(n, k, pp, qq, expected) in tests {
            assert_eq!(run(n, k, &pp, &qq), expected);
            assert_eq!(run2(n, k, &pp, &qq), expected);
        }
    }
}
