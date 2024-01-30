// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_o

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn run(a: usize, b: usize) -> usize {
    gcd(a, b)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(11, run(33, 88));
        assert_eq!(3, run(123, 777));
    }
}
