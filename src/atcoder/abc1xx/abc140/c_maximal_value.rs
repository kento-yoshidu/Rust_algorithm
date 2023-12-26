// https://atcoder.jp/contests/abc140/tasks/abc140_c

pub fn run(_n: usize, b: Vec<usize>) -> usize {
    let mut ans = b[0];

    for i in 0..b.len()-1 {
        ans += b[i].min(b[i+1]);
    }

    ans += b.iter().last().unwrap();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(9, run(3, vec![2, 5]));
        assert_eq!(6, run(2, vec![3]));
        assert_eq!(53, run(6, vec![0, 153, 10, 10, 23]));
    }
}
