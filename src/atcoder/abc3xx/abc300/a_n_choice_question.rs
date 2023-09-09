// https://atcoder.jp/contests/abc300/tasks/abc300_a

pub fn run(_n: usize, a: usize, b: usize, vec: Vec<usize>) -> usize {
    vec.iter().position(|&x| x == a+b).unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(3, 125, 175, vec![200, 300, 400]));
        assert_eq!(1, run(1, 1, 1, vec![2]));
        assert_eq!(5, run(5, 123, 456, vec![135, 246, 357, 468, 579]));
    }
}
