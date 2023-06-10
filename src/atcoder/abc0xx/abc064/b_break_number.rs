fn func(n: i32) -> bool {
    let mut tmp = n;

    loop {
        if tmp == 1 {
            return true;
        } else if tmp % 2 == 0 {
            tmp /= 2;
            continue;
        } else {
            return false;
        }
    }
}

pub fn run(n: i32) -> i32 {
    let mut result = 1;

    for i in (1..=n).rev() {
        if func(i) {
            result = i;
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, run(7));
        assert_eq!(32, run(32));
        assert_eq!(1, run(1));
        assert_eq!(64, run(100));
    }
}
