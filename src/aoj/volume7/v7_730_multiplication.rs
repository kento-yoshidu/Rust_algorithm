#[allow(dead_code)]
pub fn run(n: usize, vec: Vec<usize>) -> i32 {
    let mut ans = 0;

    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                if vec[i]*vec[j] == vec[k] {
                    ans += 1;
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(3, vec![21, 13, 273]));
        assert_eq!(0, run(3, vec![10, 5, 2]));
        assert_eq!(4, run(5, vec![4, 2, 2, 8, 16]));
        assert_eq!(120, run(10, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]));
    }
}

// https://atcoder-tags.herokuapp.com/tags/Searching/Brute-Force
