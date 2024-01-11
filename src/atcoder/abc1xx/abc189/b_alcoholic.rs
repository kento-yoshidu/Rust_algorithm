// https://atcoder.jp/contests/abc189/tasks/abc189_b

pub fn run(_n: usize, x: usize, vp: Vec<(usize, usize)>) -> isize {
    let mut current = 0.0;

    for (i, (v, p)) in vp.iter().enumerate() {
        current += *v as f64 * (*p as f64 / 100.0);

        if current > x as f64 {
            return (i + 1) as isize
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(2, 15, vec![(200, 5), (350, 3)]));
        assert_eq!(2, run(2, 10, vec![(200, 5), (350, 3)]));
        assert_eq!(-1, run(3, 1000000, vec![(1000, 100), (1000, 100), (1000, 100)]));
    }
}
