// https://atcoder.jp/contests/arc105/tasks/arc105_b

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut ans = a[0];

    for b in 1..a.len() {
        ans = ans.min(gcd(ans, a[b]));
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(3, vec![2, 6, 6]));
        assert_eq!(42, run(15, vec![546, 3192, 1932, 630, 2100, 4116, 3906, 3234, 1302, 1806, 3528, 3780, 252, 1008, 588]));
    }
}
