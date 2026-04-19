// https://atcoder.jp/contests/abc446/tasks/abc446_b

fn run(_n: usize, m: usize, lx: Vec<(usize, Vec<usize>)>) -> Vec<usize> {
    let mut ans = Vec::new();

    let mut arr = vec![false; m + 1];

    'outer: for (_l, x) in lx {
        for i in x {
            if !arr[i] {
                ans.push(i);
                arr[i] = true;
                continue 'outer;
            }
        }

        ans.push(0);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, Vec<usize>)>, Vec<usize>);

    #[test]
    fn abc446_b() {
        let tests = [
            TestCase(4, 5, vec![(3, vec![3, 1, 2]), (3, vec![3, 2, 1]), (2, vec![2, 3]), (4, vec![2, 5, 3, 1])], vec![3, 2, 0, 5]),
            TestCase(6, 5, vec![(1, vec![3]), (2, vec![3, 5]), (5, vec![5, 3, 1, 4, 2]), (5, vec![5, 1, 3, 4, 2]), (5, vec![3, 4, 1, 5, 2]), (5, vec![5, 1, 3, 2, 4])], vec![3, 5, 1, 4, 2, 0]),
        ];

        for TestCase(n, m, lx, expected) in tests {
            assert_eq!(run(n, m, lx), expected);
        }
    }
}
