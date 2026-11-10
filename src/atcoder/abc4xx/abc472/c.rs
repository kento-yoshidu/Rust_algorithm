// https://atcoder.jp/contests/abc472/tasks/abc472_c

use std::collections::VecDeque;

fn run(n: usize, m: usize, k: usize, a: Vec<usize>) -> Vec<&'static str> {
    let mut ans = Vec::new();

    let mut sum = 0;
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for i in 0..n {
        while let Some(&(day, cal)) = queue.front() {
            if day + m <= i {
                sum -= cal;
                queue.pop_front();
            } else {
                break;
            }
        }

        if sum + a[i] <= k {
            ans.push("Yes");
            sum += a[i];
            queue.push_back((i, a[i]));
        } else {
            ans.push("No");
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, Vec<&'static str>);

    #[test]
    fn abc472_c() {
        let tests = [
            TestCase(5, 3, 83, vec![48, 73, 59, 90, 21], vec!["Yes", "No", "No", "No", "Yes"]),
            TestCase(7, 4, 728, vec![187, 816, 349, 609, 255, 308, 175], vec!["Yes", "No", "Yes", "No", "Yes", "No", "Yes"]),
            TestCase(10, 3, 1368290936, vec![216519459, 804733999, 297250023, 775422599, 287963235, 999315644, 354987425, 974810607, 653940822, 117157941], vec!["Yes", "Yes", "Yes", "No", "Yes", "Yes", "No", "No", "Yes", "Yes"]),
        ];

        for TestCase(n, m, k, a, expected) in tests {
            assert_eq!(run(n, m, k, a), expected);
        }
    }
}
