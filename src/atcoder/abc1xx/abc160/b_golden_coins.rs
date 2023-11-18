// https://atcoder.jp/contests/abc160/tasks/abc160_b

pub fn run(x: usize) -> usize {
    let a = x / 500;
    let b = (x - a * 500) / 5;

    a * 1000 + b * 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2020, run(1024));
        assert_eq!(0, run(0));
        assert_eq!(2000000000, run(1000000000));
    }
}