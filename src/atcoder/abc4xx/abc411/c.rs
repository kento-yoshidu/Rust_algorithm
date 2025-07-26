// https://atcoder.jp/contests/abc411/tasks/abc411_c

fn run(n: usize, _q: usize, a: Vec<usize>) -> Vec<usize> {
    let mut vec = vec![false; n + 2];

    let mut count = 0;

    let mut ans = Vec::new();

    for i in a {
        vec[i] = !vec[i];

        if vec[i] {
            if !vec[i-1] && !vec[i+1] {
                count += 1;
            } else if vec[i-1] && vec[i+1] {
                count -= 1;
            }
        } else {
            if vec[i-1] && vec[i+1] {
                count += 1;
            } else if !vec[i-1] && !vec[i+1] {
                count -= 1;
            }
        }

        ans.push(count);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, Vec<usize>);

    #[test]
    fn abc411_c() {
        let tests = [
            TestCase(5, 7, vec![2, 3, 3, 5, 1, 5, 2], vec![1, 1, 1, 2, 2, 1, 1]),
            TestCase(1, 2, vec![1, 1], vec![1, 0]),
            TestCase(3, 3, vec![1, 3, 2], vec![1, 2, 1]),
        ];

        for TestCase(n, q, a, expected) in tests {
            assert_eq!(run(n, q, a), expected);
        }
    }
}
