// https://atcoder.jp/contests/abc268/tasks/abc268_a

fn run(v: &Vec<usize>) -> usize {
    let mut vec = v.clone();

    vec.sort();
    vec.dedup();

    vec.len()
}

fn run2(v: &Vec<usize>) -> usize {
    use itertools::Itertools;

    v.into_iter().unique().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<usize>, usize);

    #[test]
    fn abc268_a() {
        let tests = [
            TestCase(vec![31, 9, 24, 31, 24], 3),
            TestCase(vec![0, 0, 0, 0, 0], 1),
            TestCase(vec![1, 2, 3, 4, 5], 5),
        ];

        for TestCase(v, expected) in tests {
            assert_eq!(run(&v), expected);
            assert_eq!(run2(&v), expected);
        }
    }
}
