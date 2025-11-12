// https://atcoder.jp/contests/abc425/tasks/abc425_b

use std::collections::HashSet;

fn run(n: usize, a: Vec<isize>) -> (&'static str, Option<Vec<isize>>) {
    let mut nums: HashSet<isize> = HashSet::new();

    for num in a.iter() {
        if *num != -1 {
            if nums.get(num).is_none() {
                nums.insert(*num);
            } else {
                return ("No", None);
            }
        }
    }

    let mut ans = a.clone();
    let mut flag = vec![false; n+1];

    for num in ans.iter() {
        if *num != -1 {
            flag[*num as usize] = true;
        }
    }

    for i in 0..n {
        if ans[i] == -1 {
            let mut j = 1;

            loop {
                if !flag[j] {
                    ans[i] = j as isize;
                    flag[j] = true;
                    break;
                }

                j += 1;
            }
        }
    }

    ("Yes", Some(ans))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, (&'static str, Option<Vec<isize>>));

    #[test]
    fn abc425_b() {
        let tests = [
            TestCase(4, vec![-1, -1, 2, -1], ("Yes", Some(vec![1, 3, 2, 4]))),
            TestCase(5, vec![-1, -1, 1, -1, 1], ("No", None)),
            TestCase(7, vec![3, -1, 4, -1, 5, -1, 2], ("Yes", Some(vec![3, 1, 4, 6, 5, 7, 2]))),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
