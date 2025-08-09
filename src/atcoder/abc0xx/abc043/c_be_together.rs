// https://atcoder.jp/contests/abc043/tasks/arc059_a

fn run(_n: usize, a: &Vec<isize>) -> isize {
    let min = *a.iter().min().unwrap();
    let max = *a.iter().max().unwrap();

    let mut ans = std::isize::MAX;

    for num in min..=max {
        let mut tmp = 0;

        for i in a.iter() {
            tmp += (num - i).pow(2);
        }

        ans = ans.min(tmp);
    }

    ans
}

fn run2(_n: usize, a: &Vec<isize>) -> isize {
    let min = *a.iter().min().unwrap();
    let max = *a.iter().max().unwrap();

    (min..=max)
        .map(|num| {
            a.iter()
                .map(|i| {
                    (num - i).pow(2)
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, vec![4, 8], 8),
            TestCase(3, vec![1, 1, 3], 3),
            TestCase(3, vec![4, 2, 5], 5),
            TestCase(4, vec![-100, -100, -100, -100], 0),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, &a), expected);
            assert_eq!(run2(n, &a), expected);
        }
    }
}
