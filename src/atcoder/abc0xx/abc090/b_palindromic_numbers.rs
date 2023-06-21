#[allow(dead_code)]
pub fn run(a: i32, b: i32) -> i32 {
    let mut ans = 0;

    for i in a..=b {
        let i1 = i / 10000;
        let i2 = i / 1000 % 10;
        let i3 = i / 10 % 10;
        let i4 = i % 10;

        if i1 == i4 && i2 == i3 {
            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(11009, 11332));
        assert_eq!(612, run(31415, 92653))
    }
}
