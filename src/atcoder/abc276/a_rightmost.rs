pub fn run(input: &str) -> i8 {
    let mut result: i8 = -1;

    for (i, c) in input.chars().enumerate() {
        if c == 'a' {
            result = i as i8 + 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run("abcdaxayz"), 7);
        assert_eq!(run("bcbbbz"), -1);
        assert_eq!(run("aaaaa"), 5);
    }
}
