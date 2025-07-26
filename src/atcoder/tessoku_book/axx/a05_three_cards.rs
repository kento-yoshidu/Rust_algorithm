// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_e

fn run(n: usize, k: usize) -> usize {
    let mut count = 0;

    for x in 1..=n {
        for y in 1..=n {
            if x + y >= k {
                break;
            }

            if k - x - y <= n {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize);

    #[test]
    fn tessoku_a05() {
        let tests = [
            TestCase(3, 6, 7),
            TestCase(3000, 4000, 6498498),
        ];

        for TestCase(n, k, expected) in tests {
            assert_eq!(run(n, k), expected);
        }
    }
}
