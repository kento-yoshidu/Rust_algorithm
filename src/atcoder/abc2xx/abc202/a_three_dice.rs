// https://atcoder.jp/contests/abc202/tasks/abc202_a

pub fn run(a: usize, b: usize, c: usize) -> usize {
    21 - (a + b + c)
}

pub fn run2(vec: Vec<usize>) -> usize {
    vec.iter().map(|num| {
        7 - num
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(13, run(1, 4, 3));
        assert_eq!(6, run(5, 6, 4));
        assert_eq!(14, run(2, 3, 2));
        assert_eq!(14, run(5, 1, 1));
        assert_eq!(13, run(2, 1, 5));
        assert_eq!(10, run(5, 1, 5));
        assert_eq!(11, run(6, 2, 2));
        assert_eq!(14, run(5, 1, 1));
        assert_eq!(6, run(6, 5, 4));
        assert_eq!(9, run(2, 6, 4));
    }

    #[test]
    fn test2() {
        assert_eq!(13, run2(vec![1, 4, 3]));
        assert_eq!(6, run2(vec![5, 6, 4]));
        assert_eq!(14, run2(vec![2, 3, 2]));
        assert_eq!(14, run2(vec![5, 1, 1]));
        assert_eq!(13, run2(vec![2, 1, 5]));
        assert_eq!(10, run2(vec![5, 1, 5]));
        assert_eq!(11, run2(vec![6, 2, 2]));
        assert_eq!(14, run2(vec![5, 1, 1]));
        assert_eq!(6, run2(vec![6, 5, 4]));
        assert_eq!(9, run2(vec![2, 6, 4]));
    }
}
