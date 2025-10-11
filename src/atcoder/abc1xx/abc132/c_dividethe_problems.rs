// https://atcoder.jp/contests/abc132/tasks/abc132_c

fn run(k: usize, d: Vec<usize>) -> usize {
    let mut vec = d.clone();
    vec.sort();

    let abc = vec[k/2-1];
    let arc = vec[k/2];

    arc - abc
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc132_c() {
        let tests = [
            TestCase(6, vec![9, 1, 4, 4, 6, 7], 2),
            TestCase(8, vec![9, 1, 14, 5, 5, 4, 4, 14], 0),
            TestCase(14, vec![99592, 10342, 29105, 78532, 83018, 11639, 92015, 77204, 30914, 21912, 34519, 80835, 100000, 1], 42685),
        ];

        for TestCase(k, d, expected) in tests {
            assert_eq!(run(k, d), expected);
        }
    }
}
