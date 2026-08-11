// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/4/ITP1_4_D

fn run(_n: usize, a: Vec<isize>) -> (isize, isize, isize) {
    (*a.iter().min().unwrap(), *a.iter().max().unwrap(), a.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, (isize, isize, isize));

    #[test]
    fn itp1_4_c() {
        let tests = [
            TestCase(5, vec![10, 1, 5, 4, 17], (1, 17, 37)),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
