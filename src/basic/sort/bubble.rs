pub fn run(vec: &mut Vec<usize>) -> Vec<usize> {
    for i in 0..vec.len() {
        for j in i+1..vec.len() {
            if vec[j] < vec[i] {
                let left = vec[i];
                let right = vec[j];

                vec[i] = right;
                vec[j] = left;
            }
        }
    }

    vec.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ans = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        assert_eq!(ans, run(&mut vec![8, 1, 6, 2, 7, 3, 9, 4, 5]));
        assert_eq!(ans, run(&mut vec![1, 9, 4, 5, 2, 3, 8, 7, 6]));
        assert_eq!(ans, run(&mut vec![6, 5, 8, 9, 7, 2, 4, 3, 1]));
        assert_eq!(ans, run(&mut vec![9, 8, 7, 6, 5, 4, 3, 2, 1]));
        assert_eq!(ans, run(&mut vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }
}
