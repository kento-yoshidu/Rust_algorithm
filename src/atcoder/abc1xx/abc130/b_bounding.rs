#[allow(dead_code)]
pub fn run(x: i32, vec: Vec<i32>) -> i32 {
    let mut current = 0;
    let mut ans = 0;

    for i in vec.iter() {
        if current <= x {
            current += i;
            ans += 1;
        } else {
            break;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(6, vec![3, 4, 5]));
        assert_eq!(4, run(9, vec![3, 3, 3, 3]));
    }
}
