// https://atcoder.jp/contests/abc301/tasks/abc301_b

pub fn run(n: usize, a: Vec<isize>) -> Vec<isize> {
    let mut ans = Vec::new();

    for i in 0..n-1 {
        if a[i] > a[i+1] {
            for i in (a[i+1]..=a[i]).rev() {
                ans.push(i);
            }
        } else {
            for i in a[i]..=a[i+1] {
                ans.push(i)
            }
        }
    }

    ans.dedup();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, Vec<isize>);
    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![2, 5, 1, 2], vec![2, 3, 4, 5, 4, 3, 2, 1, 2]),
            TestCase(6, vec![3, 4, 5, 6, 5, 4], vec![3, 4, 5, 6, 5, 4]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
