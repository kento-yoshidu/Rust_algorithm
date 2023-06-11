#[allow(dead_code)]
pub fn run(k: usize, vec: &mut Vec<i32>) -> i32 {
    vec.sort();

    let abc = vec[k/2-1];
    let arc = vec[k/2];

    arc - abc
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(6, &mut vec![9, 1, 4, 4, 6, 7]));
        assert_eq!(0, run(8, &mut vec![9, 1, 14, 5, 5, 4, 4, 14]));
        assert_eq!(42685, run(14, &mut vec![99592, 10342, 29105, 78532, 83018, 11639, 92015, 77204, 30914, 21912, 34519, 80835, 100000, 1]));
    }
}
