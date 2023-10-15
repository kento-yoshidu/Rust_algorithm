// https://atcoder.jp/contests/abc221/tasks/abc221_a

pub fn run(a: u32, b: u32) -> u32 {
    32_u32.pow(a - b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1024, run(6, 4));
        assert_eq!(1, run(1, 1));
    }
}
