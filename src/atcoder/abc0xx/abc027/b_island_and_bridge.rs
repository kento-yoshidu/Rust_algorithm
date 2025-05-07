// https://atcoder.jp/contests/abc027/tasks/abc027_b

fn run(n: usize, a: Vec<usize>) -> isize {
    let sum: usize = a.iter().sum();

    if sum % n != 0 {
        return -1;
    }

    let avg = sum / n;

    // 島の数
    let mut island = 0;

    // 人の数
    let mut count = 0;

    // 橋の数
    let mut ans = 0;

    for x in a {
        island += 1;
        count += x;

        if avg * island == count {
            ans += island - 1;

            island = 0;
            count = 0;
        }
    }

    ans as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![1, 2, 3], 2),
            TestCase(5, vec![2, 0, 0, 0, 3], 3),
            TestCase(2, vec![0, 99], -1),
            TestCase(4, vec![0, 0, 0, 0], 0),
            TestCase(3, vec![3, 0, 0], 2),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}

// https://kenkoro.hatenablog.com/entry/2019/04/07/031341
