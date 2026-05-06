// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ez

fn run(n: usize, a: Vec<usize>) -> usize {
    let mut ans = 0;

    for i in 0..n {
        for j in i+1..n {
            ans = ans.max(a[i] + a[j])
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn tessoku_c02() {
        let tests = [
            TestCase(5, vec![120, 150, 100, 200, 100], 350),
            TestCase(10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 19),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
