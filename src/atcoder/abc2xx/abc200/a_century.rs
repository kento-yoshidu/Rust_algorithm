// https://atcoder.jp/contests/abc200/tasks/abc200_a

pub fn run(n: usize) -> usize {
    (n as f64 / 100.0).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(21, run(2021));
        assert_eq!(2, run(200));
    }
}
