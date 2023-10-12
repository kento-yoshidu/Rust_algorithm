// https://atcoder.jp/contests/abc257/tasks/abc257_a

pub fn run(n: usize, x: usize) -> char {
    ('A'..).nth((x - 1) / n).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!('C', run(1, 3));
        assert_eq!('F', run(2, 12));
    }
}
