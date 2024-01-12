// https://atcoder.jp/contests/abc053/tasks/arc068_a

fn run(x: usize) -> usize {
    let count = (x / 11) * 2;

    if x % 11 == 0 {
        count
    } else if x % 11 > 6 {
        count + 2
    } else {
        count + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(7));
        assert_eq!(4, run(22));
        assert_eq!(27217477801, run(149696127901));
    }
}
