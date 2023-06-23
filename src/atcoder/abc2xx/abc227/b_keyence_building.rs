// https://atcoder.jp/contests/abc227/tasks/abc227_b

#[allow(dead_code)]
pub fn run(n: i32, vec: Vec<i32>) -> i32 {
    let mut count = 0;

    for i in 0..vec.len() {
        'inner: for a in 1..vec[i] {
            for b in 1..vec[i] {
                if 4*a*b + 3*a + 3*b == vec[i] {
                    count += 1;
                    break 'inner;
                }
            }
        }
    }

    n - count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(3, vec![10, 20, 39]));
        assert_eq!(3, run(5, vec![666, 777, 888, 777, 666]));
    }
}
