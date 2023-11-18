// https://atcoder.jp/contests/abc329/tasks/abc329_b

pub fn run(_n: usize, a: Vec<usize>) -> usize {
    let mut vec = a.clone();

    vec.sort();
    vec.dedup();
    vec.reverse();

    vec[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(5, vec![2, 1, 3, 3, 2]));
        assert_eq!(3, run(4, vec![4, 3, 2, 1]));
        assert_eq!(18, run(8, vec![22, 22, 18, 16, 22, 18, 18, 22]));
    }
}
