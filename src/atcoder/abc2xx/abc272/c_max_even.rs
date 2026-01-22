// https://atcoder.jp/contests/abc272/tasks/abc272_c

use itertools::Itertools;

fn run(_n: usize, a: Vec<isize>) -> isize {
    let (even, odd): (Vec<isize>, Vec<isize>) =
        a.into_iter()
            .partition(|num| {
                *num % 2 == 0
            });

    let even: Vec<isize> = even.into_iter().sorted().rev().collect();
    let odd: Vec<isize> = odd.into_iter().sorted().rev().collect();

    let mut ans = -1;

    if even.len() > 1 {
        ans = ans.max(even[0] + even[1]);
    }

    if odd.len() > 1 {
        ans = ans.max(odd[0] + odd[1]);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, isize);

    #[test]
    fn abc272_c() {
        let tests = [
            TestCase(3, vec![2, 3, 4], 6),
            TestCase(2, vec![1, 0], -1),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
