// https://atcoder.jp/contests/abc109/tasks/abc109_c

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn run(n: usize, x: isize, v: &Vec<isize>) -> isize {
    let vec: Vec<_> = v.iter().collect();

    let mut ans = (x - *vec[0]).abs();

    (1..n-1)
        .for_each(|i| {
            ans =  gcd(ans, (x - *vec[i]).abs());
        });

    ans
}

fn run2(_n: usize, x: isize, v: &Vec<isize>) -> isize {
    v.iter()
        .skip(1)
        .fold((x - &v[0]).abs(), |state, num| {
            gcd(state, (x - num).abs())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, isize, Vec<isize>, isize);

    #[test]
    fn abc109_c() {
        let tests = [
            TestCase(3, 3, vec![1, 7, 11], 2),
            TestCase(3, 81, vec![33, 105, 57], 24),
            TestCase(1, 1, vec![1000000000], 999999999),
        ];

        for TestCase(n, x, v, expected) in tests {
            assert_eq!(run(n, x, &v), expected);
            assert_eq!(run2(n, x, &v), expected);
        }
    }
}
