// https://atcoder.jp/contests/abc431/tasks/abc431_b

fn run(x: usize, n: usize, w: Vec<usize>, _q: usize, p: Vec<usize>) -> Vec<usize> {
    p.into_iter()
        .scan((x, vec![false; n]), |(acc, flag), i| {
            if flag[i-1] {
                flag[i-1] = false;
                *acc -= w[i-1];
            } else {
                flag[i-1] = true;
                *acc += w[i-1];
            }
            Some(*acc)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc431_b() {
        let tests = [
            TestCase(31, 4, vec![15, 92, 65, 35], 4, vec![3, 1, 4, 1], vec![96, 111, 146, 131]),
            TestCase(41, 10, vec![73, 8, 55, 26, 97, 48, 37, 47, 35, 55], 15, vec![1, 2, 7, 1, 6, 3, 10, 8, 4, 8, 1, 5, 9, 9, 3], vec![ 114, 122, 159, 86, 134, 189, 244, 291, 317, 270, 343, 440, 475, 440, 385]),
        ];

        for TestCase(x, n, w, q, p, expected) in tests {
            assert_eq!(run(x, n, w, q, p), expected);
        }
    }
}
