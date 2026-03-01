// https://atcoder.jp/contests/abc446/tasks/abc446_c

use std::collections::VecDeque;

fn run(_n: usize, ndab: Vec<(usize, usize, Vec<isize>, Vec<isize>)>) -> Vec<isize> {
    let mut ans = Vec::new();

    for (n, d, a, b) in ndab {
        let mut q: VecDeque<(usize, isize)> = VecDeque::new();
        let mut total: isize = 0;

        for i in 0..n {
            q.push_back((i, a[i]));
            total += a[i];

            let mut need = b[i];

            while need > 0 {
                let (day, mut cnt) = q.pop_front().unwrap();

                if cnt > need {
                    cnt -= need;
                    total -= need;
                    need = 0;

                    q.push_front((day, cnt));
                } else {
                    need -= cnt;
                    total -= cnt;
                }
            }

            while let Some(&(day, cnt)) = q.front() {
                if i - day >= d {
                    total -= cnt;

                    q.pop_front();
                } else {
                    break;
                }
            }
        }

        ans.push(total);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize, Vec<isize>, Vec<isize>)>, Vec<isize>);

    #[test]
    fn abc446_c() {
        let tests = [
            TestCase(3, vec![(3, 1, vec![7, 2, 3], vec![1, 3, 2]), (3, 2, vec![7, 2, 3], vec![1, 3, 2]), (2, 1, vec![2, 1], vec![1, 2])], vec![3, 5, 0]),
        ];

        for TestCase(n, ndab, expected) in tests {
            assert_eq!(run(n, ndab), expected);
        }
    }
}
