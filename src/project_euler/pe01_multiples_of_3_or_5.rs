// https://projecteuler.net/problem=1

pub fn run() -> usize {
    (3..1000).filter(|num| {
        num % 3 == 0 || num % 5 == 0
    }).sum()
}

fn calc(n: usize, end: usize) -> usize {
    let p = end / n;

    n * p * (p + 1) / 2
}

fn run2() -> usize {
    calc(3, 999) + calc(5, 999) - calc(15, 999)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(233168, run());
    }

    #[test]
    fn test2() {
        assert_eq!(233168, run2());
    }
}
