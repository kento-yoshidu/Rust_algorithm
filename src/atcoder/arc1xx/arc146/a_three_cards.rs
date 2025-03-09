// https://atcoder.jp/contests/arc146/tasks/arc146_a

use itertools::Itertools;

fn run(_n: usize, a: Vec<usize>) -> usize {
    let vec: Vec<usize> = a.into_iter().sorted().rev().take(3).collect();

    let mut ans = 0;

    for i in 0..3 {
        for j in 0..3 {
            if i == j {
                continue;
            }

            for k in 0..3 {
                if j == k || i == k {
                    continue;
                }

                let str = format!("{}{}{}", vec[i], vec[j], vec[k]);

                ans = ans.max(str.parse::<usize>().unwrap());
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![1, 4, 3, 5, 8], 854),
            TestCase(8, vec![813, 921, 481, 282, 120, 900, 555, 409], 921900813),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
