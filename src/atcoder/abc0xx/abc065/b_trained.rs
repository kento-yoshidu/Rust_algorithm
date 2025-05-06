// https://atcoder.jp/contests/abc065/tasks/abc065_b

fn run(n: usize, a: Vec<usize>) -> isize {
    let mut count = 0;
    let mut point = 0;

    loop {
        if a[point] == 2 {
            return (count+1) as isize
        }

        count += 1;
        point = a[point]-1;

        if count == n {
            return -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec![3, 1, 2], 2),
            TestCase(4, vec![3, 4, 1, 2], -1),
            TestCase(5, vec![3, 3, 4, 2, 4], 3),
        ];

        for TestCase(n, m, expected) in tests {
            assert_eq!(run(n, m), expected);
        }
    }
}
