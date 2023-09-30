// https://atcoder.jp/contests/abc129/tasks/abc129_a

pub fn run(p: i32, q: i32, r: i32) -> i32 {
    let mut vec = vec![p, q, r];

    vec.sort();

    vec[0] + vec[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(1, 3, 4));
        assert_eq!(5, run(3, 2, 3));
    }
}
