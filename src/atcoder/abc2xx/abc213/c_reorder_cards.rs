// https://atcoder.jp/contests/abc213/tasks/abc213_c

use itertools::Itertools;

fn run(_h: usize, _w: usize, _n: usize, ab: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let (a, b): (Vec<usize>, Vec<usize>) = ab.iter().cloned().unzip();

    let a: Vec<usize> = a.into_iter().sorted().dedup().collect();
    let b: Vec<usize> = b.into_iter().sorted().dedup().collect();

    ab.into_iter()
        .map(|(aa, bb)| {
            let x = a.binary_search(&aa).unwrap() + 1;
            let y = b.binary_search(&bb).unwrap() + 1;

            (x, y)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<(usize, usize)>, Vec<(usize, usize)>);

    #[test]
    fn abc213_c() {
        let tests = [
            TestCase(4, 5, 2, vec![(3, 2), (2, 5)], vec![(2, 1), (1, 2)]),
            TestCase(562681294, 868338948, 1, vec![(367409829, 829122210)], vec![(1, 1)]),
            TestCase(512298012, 821282085, 6, vec![(369698124, 504910482), (348980485, 143258872), (56353438, 433793995), (400812297, 590552197), (440112681, 326929237), (198468805, 562404037)], vec![(4, 4), (3, 1), (1, 3), (5, 6), (6, 2), (2, 5)]),
            TestCase(1000000000, 1000000000, 10, vec![(1, 1), (10, 10), (100, 100), (1000, 1000), (10000, 10000), (100000, 100000), (1000000, 1000000), (10000000, 10000000), (100000000, 100000000), (1000000000, 1000000000)], vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7), (8, 8), (9, 9), (10, 10)]),
        ];

        for TestCase(h, w, n, ab, expected) in tests {
            assert_eq!(run(h, w, n, ab), expected);
        }
    }
}
