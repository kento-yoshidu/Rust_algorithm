// https://atcoder.jp/contests/abc294/tasks/abc294_c

fn run(_n: usize, _m: usize, a: Vec<usize>, b: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let vec_a: Vec<(usize, char)> = a.into_iter().map(|num| (num, 'A')).collect();
    let vec_b: Vec<(usize, char)> = b.into_iter().map(|num| (num, 'B')).collect();

    let mut vec = vec![vec_a, vec_b].concat();

    vec.sort_by(|a, b| a.0.cmp(&b.0));

    let ans_a: Vec<usize> = vec.iter()
        .enumerate()
        .filter(|(_, num)| num.1 == 'A')
        .map(|(i, _)| i + 1)
        .collect();

    let ans_b: Vec<usize> = vec.iter()
        .enumerate()
        .filter(|(_, num)| num.1 == 'B')
        .map(|(i, _)| i + 1)
        .collect();

    (ans_a, ans_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>, (Vec<usize>, Vec<usize>));

    #[test]
    fn test() {
        let tests = [
            TestCase(4, 3, vec![3, 14, 15, 92], vec![6, 53, 58], (vec![1, 3, 4, 7], vec![2, 5, 6])),
            TestCase(4, 4, vec![1, 2, 3, 4,], vec![100, 200, 300, 400], (vec![1, 2, 3, 4], vec![5, 6, 7, 8])),
            TestCase(8, 12, vec![3, 4, 10, 15, 17, 18, 22, 30], vec![5, 7, 11, 13, 14, 16, 19, 21, 23, 24, 27, 28], (vec![1, 2, 5, 9, 11, 12, 15, 20], vec![3, 4, 6, 7, 8, 10, 13, 14, 16, 17, 18, 19])),
        ];

        for TestCase(n, m, a, b, expected) in tests {
            assert_eq!(run(n, m, a, b), expected);
        }
    }
}
