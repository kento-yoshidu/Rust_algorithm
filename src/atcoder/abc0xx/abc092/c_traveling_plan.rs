// https://atcoder.jp/contests/abc092/tasks/arc093_a

pub fn run(n: usize, a: Vec<isize>) -> Vec<isize> {
    let mut vec = a.clone();
    vec.insert(0, 0);
    vec.push(0);

    let sum: isize = vec.windows(2)
        .map(|arr| {
            (arr[0] - arr[1]).abs()
        })
        .sum();

    let mut ans = Vec::new();

    for i in 1..=n {
        ans.push(sum + (vec[i-1] - vec[i+1]).abs() - ((vec[i-1] - vec[i]).abs() + (vec[i] - vec[i+1]).abs()))
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, Vec<isize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![3, 5, -1], vec![12, 8, 10]),
            TestCase(5, vec![1, 1, 1, 2, 0], vec![4, 4, 4, 2, 4]),
            TestCase(6, vec![-679, -2409, -3258, 3095, -3291, -4462], vec![21630 ,21630 ,19932 ,8924 ,21630 ,19288]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
