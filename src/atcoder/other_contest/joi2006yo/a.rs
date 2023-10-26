// https://atcoder.jp/contests/joi2006yo/tasks/joi2006yo_a

pub fn run(_n: usize, vec: Vec<(usize, usize)>) -> Vec<usize> {
    let mut a = 0;
    let mut b = 0;

    vec.iter()
        .for_each(|t| {
            if t.0 > t.1 {
                a += t.0 + t.1;
            } else if t.0 < t.1 {
                b += t.0 + t.1
            } else {
                a += t.0;
                b += t.0;
            }
        });

    vec![a, b]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![19, 8], run(3, vec![(9, 1), (5, 4), (0, 8)]));
        assert_eq!(vec![20, 0], run(3, vec![(9, 1), (5, 4), (1, 0)]));
        assert_eq!(vec![15, 14], run(3, vec![(9, 1), (5, 5), (1, 8)]));
    }
}
