use std::collections::HashSet;

#[allow(dead_code)]
pub fn run(a: i32, b: i32, c: i32) -> usize {
    let vec = vec![a, b, c];

    let u: HashSet<i32> = vec.into_iter().collect();

    u.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(3, 1, 4));
        assert_eq!(2, run(3, 3, 33));
    }
}
