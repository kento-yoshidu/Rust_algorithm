pub fn run(num: u32) -> u32 {
    let mut total = 0;
    let mut day = 0;

    for i in 1..=num {
        if total < num {
            total += i;
            day += 1;
        } else {
            break;
        }
    }

    day
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(5, run(12));
        assert_eq!(447, run(100128));
    }
}
