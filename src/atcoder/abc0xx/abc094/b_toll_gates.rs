// https://atcoder.jp/contests/abc094/tasks/abc094_b

pub fn run(n: usize, _m: usize, x: usize, a: Vec<usize>) -> usize {
    let to_start = (1..x).filter(|num| {
        a.contains(num)
    }).count();

    let to_end = (x+1..=n).filter(|num| {
        a.contains(num)
    }).count();

    to_start.min(to_end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(5, 3, 3, vec![1, 2, 4]));
        assert_eq!(0, run(7, 3, 2, vec![4, 5, 6]));
        assert_eq!(3, run(10, 7, 5, vec![1, 2, 3, 4, 6, 8, 9]));
    }
}
