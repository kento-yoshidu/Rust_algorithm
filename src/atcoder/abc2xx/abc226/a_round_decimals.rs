// https://atcoder.jp/contests/abc226/tasks/abc226_a

pub fn run(x: f32) -> usize {
    x.round() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(3.456));
        assert_eq!(100, run(99.500));
        assert_eq!(0, run(0.000));
    }
}
