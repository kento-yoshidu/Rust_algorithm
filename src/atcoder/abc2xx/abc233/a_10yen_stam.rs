// https://atcoder.jp/contests/abc233/tasks/abc233_a

pub fn run(x: usize, y: usize) -> usize {
    if y < x {
        return 0
    }

    ((y - x) as f64 / 10.0).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(80, 94));
        assert_eq!(0, run(1000, 63));
        assert_eq!(48, run(270, 750));
    }
}
