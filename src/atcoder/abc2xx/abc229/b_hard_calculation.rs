// https://atcoder.jp/contests/abc229/tasks/abc229_b

fn calc(a: usize, b: usize) -> bool {
    if a == 0 || b == 0 {
        return true
    }

    if a%10 + b %10 >= 10 {
        return false
    }

    calc(a/10, b/10)
}

pub fn run(a: usize, b: usize) -> String {
    if calc(a, b) == true {
        String::from("Easy")
    } else {
        String::from("Hard")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Hard"), run(229, 390));
        assert_eq!(String::from("Easy"), run(123456789, 9876543210));
    }
}
