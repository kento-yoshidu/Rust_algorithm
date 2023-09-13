// https://atcoder.jp/contests/abc022/tasks/abc022_a

pub fn run(_n: isize, s: isize, t: isize, w: Vec<isize>) -> usize {
    let vec = w.iter().scan(0, |state, &x| {
        *state = *state + x;
        Some(*state)
    }).collect::<Vec::<isize>>();

    vec.iter().filter(|num| {
        s <= **num && **num <= t
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(5, 60, 70, vec![50, 10, 10, 10, 10]));
        assert_eq!(2, run(5, 50, 100, vec![120, -10, -20, -30, 70]));
        assert_eq!(0, run(5, 50, 100, vec![120, -1, -1, -1, -1]));
        assert_eq!(5, run(5, 50, 100, vec![100, -1, -1, -1, -1]));
    }
}
