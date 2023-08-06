// https://atcoder.jp/contests/abc093/tasks/abc093_b

#[allow(dead_code)]
pub fn run(a: i32, b: i32, k: i32) -> Vec<usize> {
    let mut vec: Vec<usize> = Vec::new();

    for i in a ..=b {
        if i < a + k || b - k < i {
            vec.push(i.try_into().unwrap())
        }
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![3, 4, 7, 8], run(3, 8, 2));
        assert_eq!(vec![4, 5, 6, 7, 8], run(4, 8, 3));
        assert_eq!(vec![2, 3, 4, 5, 6, 7, 8, 9], run(2, 9, 100));
    }
}
