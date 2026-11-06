// https://atcoder.jp/contests/awc0031/tasks/awc0031_a

fn run(n: usize, m: usize, k: usize, vct: Vec<(usize, usize, Vec<usize>)>) -> usize {
    let mut index: Vec<usize> = (0..n).collect();

    index.sort_by(|&a, &b| vct[b].0.cmp(&vct[a].0).then(a.cmp(&b)));

    let mut count = vec![0usize; m + 1];

    for i in index.into_iter().take(k) {
        for t in &vct[i].2 {
            count[*t] += 1;
        }
    }

    count.into_iter().filter(|c| *c == k).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<(usize, usize, Vec<usize>)>, usize);

    #[test]
    fn awc0031_a() {
        let tests = [
            TestCase(3, 5, 2, vec![(100, 3, vec![1, 2, 3]), (80, 4, vec![1, 2, 4, 5]), (90, 2, vec![1, 2])], 2),
            TestCase(5, 6, 3, vec![(50, 2, vec![1, 3]), (100, 3, vec![1, 2, 3]), (100, 4, vec![1, 2, 3, 4]), (75, 3, vec![2, 3, 5]), (60, 0, vec![])], 2),
            TestCase(10, 8, 4, vec![(500, 5, vec![1, 2, 3, 4, 5]), (450, 4, vec![1, 2, 3, 6]), (480, 6, vec![1, 2, 3, 4, 5, 6]), (300, 3, vec![1, 2, 7]), (520, 4, vec![1, 2, 3, 8]), (200, 2, vec![1, 2]), (490, 5, vec![1, 2, 3, 5, 7]), (350, 4, vec![2, 3, 4, 5]), (400, 3, vec![1, 3, 4]), (100, 1, vec![1])], 3),
        ];

        for TestCase(n, m, k, vct, expected) in tests {
            assert_eq!(run(n, m, k, vct), expected);
        }
    }
}
