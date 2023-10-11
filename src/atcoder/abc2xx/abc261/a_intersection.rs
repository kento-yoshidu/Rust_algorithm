use std::cmp;

pub fn run(l1: i8, r1: i8, l2: i8, r2: i8) -> i8 {
    cmp::max(0, cmp::min(r1, r2) - cmp::max(l1, l2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(0, 3, 1, 5), 2);
        assert_eq!(run(0, 1, 4, 5), 0);
        assert_eq!(run(0, 3, 3, 7), 0);
    }
}
