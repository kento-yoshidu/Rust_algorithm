// https://atcoder.jp/contests/abc096/tasks/abc096_b

pub fn run(v: Vec<usize>, k: usize) -> usize {
    let sum: usize = v.iter().sum();

    sum + v.iter().max().unwrap() * (2 * k -1)
}

pub fn run2(v: Vec<usize>, k: usize) -> usize {
    let max = v.iter().max().unwrap();

    let rest: usize = v.iter().filter(|num| *num != max).sum();

    max * (2 * k) + rest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(30, run(vec![5, 3, 11], 1));
        assert_eq!(22, run(vec![3, 3, 4], 2));
    }

    #[test]
    fn test2() {
        assert_eq!(30, run2(vec![5, 3, 11], 1));
        assert_eq!(22, run2(vec![3, 3, 4], 2));
    }
}
