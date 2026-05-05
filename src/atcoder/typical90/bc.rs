// https://atcoder.jp/contests/typical90/tasks/typical90_bc
// Refactoring 3つ目のテストが時間かかるので計算量減らす?

fn run(n: usize, p: usize, q: usize, vec: Vec<usize>) -> usize {
    let mut ans = 0;

    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                for l in k+1..n {
                    for m in l+1..n {
                        if vec[i]%p * vec[j]%p * vec[k]%p * vec[l]%p * vec[m]% p == q {
                            ans += 1;
                        };
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

    struct TestCase(usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn typical90_bc() {
        let tests = [
            TestCase(6, 7, 1, vec![1, 2, 3, 4, 5, 6], 1),
            TestCase(10, 1, 0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 252),
        ];

        for TestCase(n, p, q, vec, expected) in tests {
            assert_eq!(run(n, p, q, vec), expected);
        }
    }
}
