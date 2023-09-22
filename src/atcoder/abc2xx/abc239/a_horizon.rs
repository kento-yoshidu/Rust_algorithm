// https://atcoder.jp/contests/abc239/tasks/abc239_a

pub fn run(h: usize) -> f64 {
    ((h * (h + 12800000)) as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(65287.9076782217, run(333));
        assert_eq!(90086.63583462311, run(634));
    }
}
