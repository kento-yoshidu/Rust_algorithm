// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_t

fn run(n: usize, a: Vec<usize>) -> usize {
    let mut ans = 0;

    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                for l in k+1..n {
                    for m in l+1..n {
                        if a[i] + a[j] + a[k] + a[l] + a[m] == 1000 {
                            ans += 1;
                        }
                    }
                }
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
    fn ma020() {
        let tests = [
            TestCase(5, vec![100, 150, 200, 250, 300], 1),
            TestCase(13, vec![243, 156, 104, 280, 142, 286, 196, 132, 128, 195, 265, 300, 130], 4),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
