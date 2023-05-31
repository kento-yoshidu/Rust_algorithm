fn calc(num: u32) -> u32 {
    let mut n = num;
    let mut result = 0;

    while n > 0 {
        result += n % 10;
        n = n / 10;
    }

    result
}

pub fn run(n: u32, a: u32, b: u32) -> u32 {
    let mut result = 0;

    for i in 1..=n {
        let res = calc(i);

        if a <= res && res <= b {
            result += i;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(45, run(14, 2, 4));
        assert_eq!(84, run(20, 2, 5));
        assert_eq!(13, run(10, 1, 2));
        assert_eq!(4554, run(100, 4, 16));
    }
}

