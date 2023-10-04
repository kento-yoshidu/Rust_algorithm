// https://atcoder.jp/contests/abc176/tasks/abc176_c

pub fn run(_n: usize, a: Vec<isize>) -> isize {
    a.iter()
        .scan(0, |state, &num| {
            *state = num.max(*state);
            Some(*state - num)
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(5, vec![2, 1, 5, 4, 3]));
        assert_eq!(0, run(0, vec![3, 3, 3, 3, 3]));
    }
}
