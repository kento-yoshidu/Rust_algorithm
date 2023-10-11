// https://atcoder.jp/contests/abc207/tasks/abc207_a

pub fn run(a: usize, b: usize, c: usize) -> usize {
    let mut vec = vec![a, b, c];

    vec.sort();
    vec.reverse();

    vec[0] + vec[1]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(9, run(3, 4 ,5));
        assert_eq!(12, run(6, 6, 6));
        assert_eq!(198, run(99, 99, 98));
    }
}
