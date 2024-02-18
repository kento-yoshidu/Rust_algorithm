// https://atcoder.jp/contests/abc341/tasks/abc341_b

pub fn run(n: usize, a: Vec<usize>, st: Vec<(usize, usize)>) -> usize {
    let mut vec = a.clone();

    for (i, (x, y)) in st.iter().enumerate() {
        let tmp = vec[i] / x;

        vec[i] -= x*tmp;
        vec[i+1] += y*tmp;
    }

    vec[n-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<(usize, usize)>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, vec![5, 7, 0, 3], vec![(2, 2), (4, 3), (5, 2)], 5),
            TestCase(10, vec![32, 6, 46, 9, 37, 8, 33, 14, 31, 5], vec![(5, 5), (3, 1), (4, 3), (2, 2), (3, 2), (3, 2), (4, 4), (3, 3), (3, 1)], 45),
        ];

        for TestCase(n, a, st, expected) in tests {
            assert_eq!(run(n, a, st), expected);
        }
    }
}
