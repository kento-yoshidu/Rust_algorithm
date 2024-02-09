// https://atcoder.jp/contests/bcu30-2018-qual/tasks/bcu30_2018_qual_a

fn run(_n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut vec = vec![0; 30];

    for num in a {
        vec[num] += 1;
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![1, 19, 3, 29, 0], vec![1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
            TestCase(2, vec![0, 0], vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
