// https://atcoder.jp/contests/abc106/tasks/abc106_b

pub fn check(n: usize) -> bool {
    let mut count = 0;

    for i in 1..=n {
        if n % i == 0 {
            count += 1;
        }
    }

    if count == 8 {
        true
    } else {
        false
    }
}

#[allow(dead_code)]
pub fn run(n: usize) -> usize {
    let mut ans = 0;

    for i in (1..=n).step_by(2) {
        if check(i) == true {
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
        assert_eq!(0, run(104));
        assert_eq!(1, run(105));
        assert_eq!(1, run(134));
        assert_eq!(2, run(135));
        assert_eq!(2, run(164));
        assert_eq!(3, run(165));
    }
}
