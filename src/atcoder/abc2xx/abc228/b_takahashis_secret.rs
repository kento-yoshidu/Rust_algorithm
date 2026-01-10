// https://atcoder.jp/contests/abc228/tasks/abc228_b

fn check(a: &Vec<usize>, mut vec: Vec<bool>, count: usize, current: usize) -> usize {
    if vec[current-1] == true {
        count
    } else {
        vec[current-1] = true;
        check(a, vec, count+1, a[current-1])
    }
}

fn run(n: usize, x: usize, a: Vec<usize>) -> usize {
    let vec = vec![false; n];

    check(&a, vec, 0, x)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn abc228_b() {
        let tests = [
            TestCase(4, 2, vec![3, 1, 1, 2], 3),
            TestCase(20, 12, vec![7, 11, 10, 1, 7, 20, 14, 2, 17, 3, 2, 5, 19, 20, 8, 14, 18, 2, 10, 10], 7),
        ];

        for TestCase(n, x, a, expected) in tests {
            assert_eq!(run(n, x, a), expected);
        }
    }
}
