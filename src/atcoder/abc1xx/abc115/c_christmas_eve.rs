#[allow(dead_code)]
pub fn run(n: usize, k: usize, vec: &mut Vec<usize>) -> usize {
    vec.sort();

    (0..=n - k).map(|i| {
        vec[i + k-1] - vec[i]
    }).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(5, 3, &mut vec![10, 15, 11, 14, 12]));
        assert_eq!(0, run(5, 3, &mut vec![5, 7, 5, 7, 7]));
    }
}
