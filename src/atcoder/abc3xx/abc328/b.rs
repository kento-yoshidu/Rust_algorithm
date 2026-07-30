// https://atcoder.jp/contests/abc328/tasks/abc328_b

use itertools::Itertools;

fn check(s: String) -> bool {
    s.chars().all_equal()
}

fn run(_n: usize, d: &Vec<usize>) -> usize {
    let mut ans = 0;

    for (index, days) in d.iter().enumerate() {
        for day in 1..=*days {
            if check(format!("{}{}", index+1, day)) {
                ans += 1;
            }
        }
    }

    ans
}

fn run2(_n: usize, d: &Vec<usize>) -> usize {
    d.into_iter()
        .enumerate()
        .map(|(index, days)| {
            (1..=*days)
                .filter(|day| {
                    check(format!("{}{}", index+1, day))
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc328_b() {
        let tests = [
            TestCase(10, vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31], 13),
            TestCase(10, vec![10, 1, 2, 3, 4, 5, 6, 7, 8, 100], 1),
            TestCase(30, vec![73, 8, 55, 26, 97, 48, 37, 47, 35, 55, 5, 17, 62, 2, 60, 23, 99, 73, 34, 75, 7, 46, 82, 84, 29, 41, 32, 31, 52, 32], 15),
        ];

        for TestCase(n, d, expected) in tests {
            assert_eq!(run(n, &d), expected);
            assert_eq!(run2(n, &d), expected);
        }
    }
}
