// https://atcoder.jp/contests/abc083/tasks/abc083_b

fn calc(num: usize) -> usize {
    let mut n = num;
    let mut result = 0;

    while n > 0 {
        result += n % 10;
        n = n / 10;
    }

    result
}

pub fn run(n: usize, a: usize, b: usize) -> usize {
    let mut result = 0;

    for i in 1..=n {
        let res = calc(i);

        if a <= res && res <= b {
            result += i;
        }
    }

    result
}

pub fn run2(n: usize, a: usize, b: usize) -> usize {
    (1..=n).filter(|num| {
        let res = calc(*num);
        a <= res && res <= b
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(45, run(14, 2, 4));
        assert_eq!(84, run(20, 2, 5));
        assert_eq!(13, run(10, 1, 2));
        assert_eq!(4554, run(100, 4, 16));
    }

    #[test]
    fn test2() {
        assert_eq!(45, run2(14, 2, 4));
        assert_eq!(84, run2(20, 2, 5));
        assert_eq!(13, run2(10, 1, 2));
        assert_eq!(4554, run2(100, 4, 16));
    }
}
