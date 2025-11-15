// https://atcoder.jp/contests/abc209/tasks/abc209_b

fn run(_n: usize, x: usize, a: Vec<usize>) -> &'static str {
    let total: usize = a.into_iter()
        .enumerate()
        .map(|(i, num)| {
            if i % 2 == 0 {
                num
            } else {
                num - 1
            }
        })
        .sum();

    if total <= x {
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
    fn abc209_b() {
        let tests = [
            TestCase(2, 3, vec![1, 3], "Yes"),
            TestCase(4, 10, vec![3, 3, 4, 4], "No"),
            TestCase(8, 30, vec![3, 1, 4, 1, 5, 9, 2, 6], "Yes"),
            TestCase(15, 775, vec![88, 57, 44, 92, 28, 66, 60, 37, 33, 52, 38, 29, 76, 8, 75], "No"),
        ];

        for TestCase(n, x, a, expected) in tests {
            assert_eq!(run(n, x, a), expected);
        }
    }
}
