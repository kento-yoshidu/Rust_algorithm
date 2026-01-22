// https://atcoder.jp/contests/joi2026yo2/tasks/joi2026_yo2_a

use itertools::Itertools;

fn run(n: usize, a: Vec<usize>) -> usize {
    let a: Vec<usize> = a.into_iter().sorted().collect();

    let mut index = 0;
    let mut diff = std::isize::MAX;

    for i in 1..=n-1 {
        if a[i - 1] == a[i] {
            continue;
        }

        let d = ((n as isize - i as isize) - i as isize).abs();

        if d < diff {
            diff = d;
            index = i;
        } else if d == diff {
            if (n - i) < (n - index) {
                index = i;
            }
        }
    }

    a[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn joi2026yo2_a() {
        let tests = [
            TestCase(3, vec![1000, 500, 800], 1000),
            TestCase(6, vec![100, 75, 41, 75, 13, 89], 89),
            TestCase(6, vec![20, 25, 12, 7, 13, 16], 16),
            TestCase(8, vec![364353982, 103422534, 437367896, 91518637, 364353982, 221490368, 437367896, 103422534], 364353982),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
