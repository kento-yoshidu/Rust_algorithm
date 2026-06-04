// https://atcoder.jp/contests/abc329/tasks/abc329_d

fn run(n: usize, _m: usize, a: Vec<usize>) -> Vec<usize> {
    let mut arr = vec![0; n];
    let mut current = (0, 0);

    let mut ans: Vec<usize> = Vec::new();

    for i in a.iter() {
        arr[i-1] += 1;

        if arr[i-1] > current.1 {
            current = (*i, current.1 + 1);
        } else if arr[i-1] == current.1 {
            current = (*i.min(&current.0), current.1);
        }

        ans.push(current.0)
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc329_d() {
        let tests = [
            TestCase(3, 7, vec![1, 2, 2, 3, 1, 3, 3], vec![1, 1, 2, 2, 1, 1, 3]),
            TestCase(100, 5, vec![100, 90, 80, 70, 60], vec![100, 90, 80, 70, 60]),
            TestCase(9, 8, vec![8, 8, 2, 2, 8, 8, 2, 2], vec![8, 8, 8, 2, 8, 8, 8, 2]),
        ];

        for TestCase(n, m, a, expected) in tests {
            assert_eq!(run(n, m, a), expected);
        }
    }
}
