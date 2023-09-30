// https://atcoder.jp/contests/abc120/tasks/abc120_b

pub fn run(a: i32, b: i32, k: i32) -> i32 {
    let mut count = 0;
    let mut result = 0;

    for i in (1..=std::cmp::min(a, b)).rev() {
        if a % i == 0 && b % i == 0 {
            count += 1;

            if count == k {
                result = i;
                break
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(8, 12, 2));
        assert_eq!(5, run(100, 50, 4));
        assert_eq!(1, run(1, 1, 1));
    }
}
