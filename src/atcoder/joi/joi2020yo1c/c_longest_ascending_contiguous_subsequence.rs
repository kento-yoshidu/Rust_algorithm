// https://atcoder.jp/contests/joi2020yo1c/tasks/joi2020_yo1c_c

fn run(n: usize, a: Vec<usize>) -> usize {
    let mut ans = 0;

    for i in 0..n {
        for j in i..n {
            let mut flag = true;

            for k in i..j {
                if a[k] > a[k+1] {
                    flag = false;
                }
            }

            if flag {
                ans = ans.max(j - i + 1);
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
            TestCase(10, vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3], 3),
            TestCase(10, vec![9, 8, 7, 6, 5, 5, 4, 3, 2, 1], 2),
            TestCase(9, vec![1, 2, 2, 12, 120, 210, 202, 1010, 2020], 6),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
