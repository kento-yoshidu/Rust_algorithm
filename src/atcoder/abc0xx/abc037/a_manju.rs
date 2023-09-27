// https://atcoder.jp/contests/abc037/tasks/abc037_a

pub fn run(a: usize, b: usize, c: usize) -> usize {
    let min = a.min(b);

    c / min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(3, 5, 6));
        assert_eq!(3, run(8, 6, 20));
    }
}
