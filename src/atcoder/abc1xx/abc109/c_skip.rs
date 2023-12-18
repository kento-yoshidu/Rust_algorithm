// https://atcoder.jp/contests/abc109/tasks/abc109_c

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn run(n: usize, x: isize, v: Vec<isize>) -> isize {
    let vec: Vec<_> = v.iter().collect();

    let mut ans = (x - *vec[0]).abs();

    (1..n-1)
        .for_each(|i| {
            ans =  gcd(ans, (x - *vec[i]).abs());
        });

    ans
}

pub fn run2(_n: usize, x: isize, v: Vec<isize>) -> isize {
    v.iter()
        .skip(1)
        .fold((x - &v[0]).abs(), |state, num| {
            gcd(state, (x - num).abs())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(3, 3, vec![1, 7, 11]));
        assert_eq!(24, run(3, 81, vec![33, 105, 57]));
        assert_eq!(999999999, run(1, 1, vec![1000000000]));
    }

    #[test]
    fn test2() {
        assert_eq!(2, run2(3, 3, vec![1, 7, 11]));
        assert_eq!(24, run2(3, 81, vec![33, 105, 57]));
        assert_eq!(999999999, run2(1, 1, vec![1000000000]));
    }
}
