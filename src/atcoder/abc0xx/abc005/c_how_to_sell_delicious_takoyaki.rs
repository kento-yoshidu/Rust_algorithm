// https://atcoder.jp/contests/abc005/tasks/abc005_3

pub fn run(t: usize, n: usize, a: Vec<usize>, _m: usize, b: Vec<usize>) -> &'static str {
    let mut cur = 0;

    for num in b.into_iter() {
        loop {
            if cur == n {
                return "no";
            }

            if num < a[cur] {
                return "no";
            }

            if a[cur] + t >= num {
                cur += 1;
                break;
            } else {
                cur += 1;
            }
        }
    }

    "yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize, Vec<usize>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(1, 3, vec![1, 2, 3], 3, vec![2, 3, 4], "yes"),
            TestCase(1, 3, vec![1, 2, 3], 3, vec![2, 3, 5], "no"),
            TestCase(1, 3, vec![1, 2, 3], 10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], "no"),
            TestCase(1, 3, vec![1, 2, 3], 3, vec![1, 2, 2], "no"),
            TestCase(2, 5, vec![1, 3, 6, 10, 15], 3, vec![4, 8, 16], "yes"),
        ];

        for TestCase(t, n, a, m, b, expected) in tests {
            assert_eq!(run(t, n, a, m, b), expected);
        }
    }
}
