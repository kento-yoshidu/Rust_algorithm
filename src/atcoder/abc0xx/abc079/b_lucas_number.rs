// https://atcoder.jp/contests/abc079/tasks/abc079_b

pub fn run(n: usize) -> usize {
    (1..n)
        .fold((2, 1), |(l, r), _| {
            (r, l+r)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(11, run(5));
        assert_eq!(939587134549734843, run(86));
    }
}
