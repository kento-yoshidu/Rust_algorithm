fn fizzbuzz(num: u64) -> bool {
    if !(num % 3 == 0 || num % 5 == 0) {
        return true;
    };

    false
}

pub fn run(num: u64) -> u64 {
    let mut total = 0;

    for i in 1..=num {
        if fizzbuzz(i) {
            total += i;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(60, run(15));
        assert_eq!(266666333332, run(1000000));
    }
}
