// https://atcoder.jp/contests/abc430/tasks/abc430_b

use std::collections::HashSet;

fn run(n: usize, m: usize, s: Vec<&str>) -> usize {
    let chars: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

    let mut set = HashSet::new();

    for i in 0..n-m+1 {
        for j in 0..n-m+1 {
            let mut arr = Vec::new();

            for k in 0..m {
                for l in 0..m {
                    arr.push(chars[i+k][j+l]);
                }
            }

            set.insert(arr.into_iter().collect::<String>());
        }
    }

    set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, usize);

    #[test]
    fn abc430_b() {
        let tests = [
            TestCase(3, 2, vec!["...", "###", "#.#"], 3),
            TestCase(10, 3, vec!["..#.......", ".###......", ".#.#......", "#####.....", "#...#.....", "......####", "......#..#", "......#...", "......#..#", "......####"], 36),
        ];

        for TestCase(n, m, s, expected) in tests {
            assert_eq!(run(n, m, s), expected);
        }
    }
}
