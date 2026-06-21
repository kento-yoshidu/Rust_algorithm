// https://atcoder.jp/contests/abc463/tasks/abc463_c

use library::lib::binary_search::lower_bound::lower_bound;

fn run(_n: usize, hl: Vec<(usize, usize)>, _q: usize, t: Vec<usize>) -> Vec<usize> {
    let mut arr: Vec<(usize, usize)> = hl.into_iter().map(|(h, l)| (l, h)).collect();
    arr.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
    arr.dedup_by_key(|x| x.0);

    let mut max = vec![0; arr.len() + 1];

    for i in (0..arr.len()).rev() {
        max[i] = max[i + 1].max(arr[i].1);
    }

    t.into_iter()
        .map(|ti| {
            let idx = lower_bound(&arr, (ti + 1, 0));
            max[idx]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc463_c() {
        let tests = [ TestCase(4, vec![(31, 4), (26, 5), (3, 5), (15, 9)], 4, vec![3, 4, 5, 6], vec![31, 26, 15, 15]),
            TestCase(10, vec![(587, 138), (772, 155), (755, 404), (519, 408), (529, 432), (169, 586), (114, 632), (249, 656), (329, 972), (299, 984)], 14, vec![443, 801, 824, 276, 399, 314, 300, 510, 311, 580, 498, 930, 359, 5], vec![ 329, 329, 329, 755, 755, 755, 755, 329, 755, 329, 329, 329, 755, 772]),
        ];

        for TestCase(n, hl, q, t, expected) in tests {
            assert_eq!(run(n, hl, q, t), expected);
        }
    }
}
