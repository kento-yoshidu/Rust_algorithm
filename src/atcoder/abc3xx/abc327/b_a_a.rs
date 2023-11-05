// https://atcoder.jp/contests/abc327/tasks/abc327_b

pub fn run(b: usize) -> isize {
    (1..=15).find(|i| {
        (*i as usize).pow(*i as u32) == b
    })
    .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(27));
        assert_eq!(-1, run(100));
        assert_eq!(10, run(10000000000));
    }
}
