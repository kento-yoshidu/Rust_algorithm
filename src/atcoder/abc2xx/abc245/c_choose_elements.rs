// https://atcoder.jp/contests/abc245/tasks/abc245_c

fn run(n: usize, k: isize, a: Vec<isize>, b: Vec<isize>) -> &'static str {
    let mut dp_a = vec![false; n];
    let mut dp_b = vec![false; n];

    dp_a[0] = true;
    dp_b[0] = true;

    for i in 0..n-1 {
        if dp_a[i] == true {
            dp_a[i+1] |= (a[i] - a[i+1]).abs() <= k;
            dp_b[i+1] |= (a[i] - b[i+1]).abs() <= k;
        }
        if dp_b[i] == true {
            dp_a[i+1] |= (b[i] - a[i+1]).abs() <= k;
            dp_b[i+1] |= (b[i] - b[i+1]).abs() <= k;
        }
    }

    if dp_a[n-1] || dp_b[n-1] {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, Vec<isize>, Vec<isize>, &'static str);

    #[test]
    fn abc245_c() {
        let tests = [
            TestCase(5, 4, vec![9, 8, 3, 7, 2], vec![1, 6, 2, 9, 5], "Yes"),
            TestCase(3, 2, vec![1, 3, 100, 101, 102], vec![1, 3, 100, 101, 102], "No"),
            TestCase(4, 1000000000, vec![1, 1, 1000000000, 1000000000], vec![1, 1000000000, 1, 1000000000], "Yes"),
        ];

        for TestCase(n, k, a, b, expected) in tests {
            assert_eq!(run(n, k, a, b), expected);
        }
    }
}
