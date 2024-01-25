// https://atcoder.jp/contests/abc320/tasks/abc320_a

pub fn run(_n: usize, p: usize, a: Vec<usize>) -> usize {
    a.iter()
        .filter(|num| **num < p )
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(4, 50, vec![80, 60, 40, 0]));
        assert_eq!(3, run(3, 90, vec![89, 89, 89]));
        assert_eq!(1, run(2, 22, vec![6, 37]));
    }
}
