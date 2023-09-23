// https://atcoder.jp/contests/abc320/tasks/abc320_a

pub fn run(a: u32, b: u32) -> u32 {
    a.pow(b) + b.pow(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(320, run(2, 8));
        assert_eq!(774840978, run(9, 9));
        assert_eq!(23401, run(5, 6));
    }
}
