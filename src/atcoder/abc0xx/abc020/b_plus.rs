// https://atcoder.jp/contests/abc020/tasks/abc020_b

pub fn run(a: usize, b: usize) -> usize {
    let num = a.to_string() + &b.to_string();

    num.parse::<usize>().unwrap() * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(246, run(1, 23));
        assert_eq!(1999998, run(999, 999));
    }
}
