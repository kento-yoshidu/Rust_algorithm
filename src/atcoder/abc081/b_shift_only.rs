fn run(s: Vec<i32>) -> i32 {
    let result: Vec<i32> = s.iter().map(|&i| {
        let mut count = 0;
        let mut tmp = i;

        loop {
            if tmp % 2 == 0 {
                tmp = tmp >> 1;
                count += 1;
            } else {
                break;
            }
        }

        count
    }).collect();

    *result.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(vec![8, 12, 40]));
        assert_eq!(0, run(vec![5, 6, 8, 10]));
        assert_eq!(8, run(vec![382253568, 723152896, 37802240, 379425024, 404894720, 471526144]));
    }
}
