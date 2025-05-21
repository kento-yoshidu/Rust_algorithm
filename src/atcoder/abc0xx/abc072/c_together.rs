
// https://atcoder.jp/contests/abc072/tasks/arc082_a

fn run(_n: isize, vec: Vec<isize>) -> usize {
    let mut ans = 0;

    let min = vec.iter().min().unwrap();
    let max = vec.iter().max().unwrap();

    for i in *min..=*max {
        let mut count = 0;

        for j in vec.iter() {
            if (*j - i).abs() <= 1 {
                count += 1;
            }
        }

        ans = ans.max(count);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(isize, Vec<isize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(7, vec![3, 1, 4, 1, 5, 9, 2], 4),
            TestCase(10, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3),
            TestCase(1, vec![99999], 1),
        ];

        for TestCase(n, vec, expected) in tests {
            assert_eq!(run(n, vec), expected);
        }
    }
}
