// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ck

fn calc(n: f64) -> f64 {
    n * n * n + n
}

fn run(n: usize) -> f64 {
    let mut left = 0.0;
    let mut right = n as f64;
    let mut mid = 0.0;


    for _ in 0..20 {
        mid = (left + right) / 2.0;

        let val = calc(mid);

        if (n as f64) < val {
            right = mid;
        } else {
            left = mid;
        }
    }

    mid
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, f64);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 1.0000019073486328),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}