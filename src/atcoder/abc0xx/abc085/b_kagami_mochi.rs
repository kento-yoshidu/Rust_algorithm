// https://atcoder.jp/contests/abc085/tasks/abc085_b


pub fn run(_n: usize, d: Vec<usize>) -> usize {
    let mut vec = d.clone();

    vec.sort();
    vec.dedup();

    vec.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(4, vec![10, 8, 8, 6]));
        assert_eq!(1, run(3, vec![15, 15, 15]));
        assert_eq!(4, run(7, vec![50, 30, 50, 100, 50, 80, 30]));
    }
}
