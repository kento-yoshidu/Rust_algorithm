// https://atcoder.jp/contests/abc118/tasks/abc118_c

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn run(_n: usize, v: Vec<usize>) -> usize {
    v.iter()
        .fold(0, |state, num| {
            gcd(state, *num)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(4, vec![2, 10, 8, 40]));
        assert_eq!(1, run(4, vec![5, 13, 8, 1000000000]));
        assert_eq!(1000000000, run(3, vec![1000000000, 1000000000, 1000000000]));
    }
}
