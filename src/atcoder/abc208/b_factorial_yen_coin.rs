fn fact (num: i32) -> i32 {
    let mut result = 1;

    for i in 1..=num {
        result = result * i;
    }

    result
}

pub fn run(input: i32) -> i32 {
    let mut total = input;
    let mut result = 0;

    for i in (1..=10).rev() {
        while total >= fact(i) {
            total = total - fact(i);
            result = result + 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, run(9));
        assert_eq!(10, run(119));
        assert_eq!(24, run(10000000));
    }
}
