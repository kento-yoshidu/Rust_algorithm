// https://atcoder.jp/contests/joi2021yo1c/tasks/joi2021_yo1c_a

pub fn run(a: isize, b: isize) -> Vec<isize> {
    let plus = a + b;
    let minus = a - b;

    if plus > minus {
        vec![plus, minus]
    } else {
        vec![minus, plus]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![-1, -3], run(-2, 1));
        assert_eq!(vec![1, -7], run(-3, -4));
        assert_eq!(vec![5, 5], run(5, 0));
    }
}
