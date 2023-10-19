// https://atcoder.jp/contests/abc077/tasks/abc077_b

pub fn run(n: usize) -> usize {
    ((n as f64).sqrt() as u32).pow(2) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(9, run(10));
        assert_eq!(81, run(81));
        assert_eq!(271821169, run(271828182));
    }
}
