// https://atcoder.jp/contests/abc235/tasks/abc235_a

pub fn run(n: usize) -> u32 {
    n.to_string().chars().map(|c| {
        let num = c.to_digit(10).unwrap();

        num * 100 + num * 10 + num
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(666, run(123));
        assert_eq!(2997, run(999));
    }
}
