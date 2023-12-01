// https://atcoder.jp/contests/abc131/tasks/abc131_b

pub fn run(n: isize, l: isize) -> isize {
    let abs_min = (0..n).map(|i| {
        (l + i).abs()
    })
    .min()
    .unwrap();

    (0..n)
        .map(|i| {
            l + i
        })
        .filter(|num| {
            num.abs() != abs_min
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(18, run(5, 2));
        assert_eq!(0, run(3, -1));
        assert_eq!(-1044, run(30, -50));
    }
}
