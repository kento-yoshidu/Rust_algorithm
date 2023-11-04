// https://atcoder.jp/contests/abc134/tasks/abc134_b

pub fn run(n: u32, d: u32) -> usize {
    (n as f64 / (d * 2 + 1) as f64).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(6, 2));
        assert_eq!(2, run(14, 3));
        assert_eq!(3, run(20, 4));
    }
}
