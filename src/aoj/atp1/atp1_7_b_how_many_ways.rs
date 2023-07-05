#[allow(dead_code)]
pub fn run(n: usize, x: usize) -> usize {
    let mut ans = 0;

    for i in 1..=n {
        for j in i+1..=n {
            for k in j+1..=n {
                if i + j + k == x {
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
        assert_eq!(2, run(5, 10));
    }
}
