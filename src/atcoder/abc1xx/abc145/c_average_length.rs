// https://atcoder.jp/contests/abc145/tasks/abc145_c

use itertools::Itertools;

fn f(n: f64) -> f64 {
    if n == 0.0 {
        1.0
    } else {
        n * f(n - 1.0)
    }
}

fn run(n: usize, xy: Vec<(isize, isize)>) -> f64 {
    let mut dist_sum = 0.0;

    for p in (0..n).permutations(n) {
        let mut dist = 0.0;

        for i in 0..n-1 {
            dist += (((xy[p[i]].0 - xy[p[i+1]].0).pow(2) + (xy[p[i]].1 - xy[p[i+1]].1).pow(2)) as f64).sqrt();
        }

        dist_sum += dist;
    }

    dist_sum / f(n as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(isize, isize)>, f64);

    #[test]
    fn abc145_c() {
        let tests = [
            TestCase(3, vec![(0, 0), (1, 0), (0, 1)], 2.2761423749153966),
            TestCase(2, vec![(-879, 981), (-866, 890)], 91.92388155425118),
            TestCase(8, vec![(-406, 10), (512, 859), (494, 362), (-955, -475), (128, 553), (-986, -885), (763, 77), (449, 310)], 7641.981782438662),
        ];

        for TestCase(n, xy, expected) in tests {
            assert_eq!(run(n, xy), expected);
        }
    }
}
