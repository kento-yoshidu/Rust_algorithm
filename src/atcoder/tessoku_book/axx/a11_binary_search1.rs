pub fn run(n: usize, x: usize, a: Vec<usize>) -> usize {
    let mut left = 1;
    let mut right = n;

    loop {
        let middle = (left + right) / 2;

        if x == a[middle] {
            return middle + 1;
        }

        if x < a[middle] {
            right = middle - 1
        } else {
            left = middle + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(11, run(15, 47, vec![11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67]));
        assert_eq!(8, run(10, 80, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]));
    }
}
