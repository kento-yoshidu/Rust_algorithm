// https://atcoder.jp/contests/abc143/tasks/abc143_a

pub fn run(a: usize, b: usize) ->usize {
    if a <= b * 2 {
        0
    } else {
        a - b * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(12, 4));
        assert_eq!(0, run(20, 15));
        assert_eq!(0, run(20, 30));
    }
}
