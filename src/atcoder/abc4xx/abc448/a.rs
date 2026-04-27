// https://atcoder.jp/contests/abc448/tasks/abc448_a

fn run(_n: usize, x: usize, a: Vec<usize>) -> Vec<usize> {
    let mut ans = Vec::new();
    let mut cur = x;

    for num in a {
        if num < cur {
            cur = num;
            ans.push(1);
        } else {
            ans.push(0);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc448_a() {
        let tests = [
            TestCase(5, 10, vec![6, 4, 7, 1, 3], vec![1, 1, 0, 1, 0]),
            TestCase(1, 1, vec![1], vec![0]),
            TestCase(8, 20, vec![9, 19, 14, 17, 17, 4, 18, 4], vec![1, 0, 0, 0, 0, 1, 0, 0 ]),
        ];

        for TestCase(n, x, a, expected) in tests {
            assert_eq!(run(n, x, a), expected);
        }
    }
}
