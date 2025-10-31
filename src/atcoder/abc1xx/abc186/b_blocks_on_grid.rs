// https://atcoder.jp/contests/abc186/tasks/abc186_b

fn run(_h: usize, _w: usize, a: Vec<Vec<usize>>) -> usize {
    let min = a.iter()
        .map(|v| {
            v.iter().min().unwrap()
        })
        .min()
        .unwrap();

    a.iter()
        .map(|v| {
            v.iter().map(|num| {
                num - min
            })
            .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<Vec<usize>>, usize);

    #[test]
    fn abc186_b() {
        let tests = [
            TestCase(2, 3, vec![vec![2, 2, 3], vec![3, 2, 2]], 2),
            TestCase(3, 3, vec![vec![99, 99, 99], vec![99, 0, 99], vec![99, 99, 99]], 792),
            TestCase(3, 2, vec![vec![4, 4], vec![4, 4], vec![4, 4]], 0),
        ];

        for TestCase(h, w, a, expected) in tests {
            assert_eq!(run(h, w, a), expected);
        }
    }
}
