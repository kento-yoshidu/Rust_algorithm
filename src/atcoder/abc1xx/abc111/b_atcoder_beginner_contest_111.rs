// https://atcoder.jp/contests/abc111/tasks/abc111_b

#[allow(dead_code)]
pub fn run(n: u32) -> u32 {
    for i in n..999 {
        if i/100 == i%100/10 && i/100 == i%10 {
            return i;
        }
    }

    999
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(111, run(111));
        assert_eq!(222, run(112));
        assert_eq!(777, run(750));
    }
}
