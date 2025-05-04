// https://atcoder.jp/contests/abc299/tasks/abc299_c

use itertools::Itertools;

fn run(_n: usize, s: &str) -> isize {
    let run_length: Vec<(usize, char)> = s.chars().dedup_with_count().collect();

    run_length
        .iter()
        .enumerate()
        .filter_map(|(i, (count, c))| {
            if *c == 'o' {
                let left = i > 0 && run_length[i-1].1 == '-';
                let right = i+1 < run_length.len() && run_length[i+1].1 == '-';

                if left || right {
                    return Some(count);
                }
            }
            None
        })
        .max()
        .map(|x| *x as isize)
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, isize);

    #[test]
    fn test() {
        let tests = [
            TestCase(10, "o-oooo---o", 4),
            TestCase(1, "-", -1),
            TestCase(30, "-o-o-oooo-oo-o-ooooooo--oooo-o", 7),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
