// https://atcoder.jp/contests/abc289/tasks/abc289_d

fn run(_n: usize, a: Vec<usize>, _m: usize, b: Vec<usize>, x: usize) -> &'static str {
    let mut dp = vec![false; x+1];
    let mut mochi = vec![false; x+1];

    for m in b {
        mochi[m] = true;
    }

    dp[0] = true;

    for i in 1..=x {
        if mochi[i] {
            continue;
        }

        for &j in a.iter() {
            if i >= j && dp[i - j] {
                dp[i] = true;
                break;
            }
        }
    }

    if dp[x] {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize, Vec<usize>, usize, &'static str);

    #[test]
    fn abc289_d() {
        let tests = [
            TestCase(3, vec![3, 4, 5], 4, vec![4, 5, 6, 8], 15, "Yes"),
            TestCase(4, vec![2, 3, 4, 5], 4, vec![3, 4, 5, 6], 8, "No"),
            TestCase(4, vec![2, 5, 7, 8], 5, vec![2, 9, 10, 11, 19], 20, "Yes"),
        ];

        for TestCase(n, a, m, b, x, expected) in tests {
            assert_eq!(run(n, a, m, b, x), expected);
        }
    }
}
