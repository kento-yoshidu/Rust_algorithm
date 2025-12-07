// https://atcoder.jp/contests/abc435/tasks/abc435_b

fn run(n: usize, a: Vec<usize>) -> usize {
    let mut ans = 0;

    for i in 0..n {
        'outer: for j in i+1..n {
            let sum: usize = a[i..=j].iter().sum();

            for k in i..=j {
                if sum % a[k] == 0 {
                    continue 'outer;
                }
            }

            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc435_b() {
        let tests = [
            TestCase(5, vec![8, 6, 10, 5, 7], 6),
            TestCase(3, vec![1, 1, 1], 0),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
