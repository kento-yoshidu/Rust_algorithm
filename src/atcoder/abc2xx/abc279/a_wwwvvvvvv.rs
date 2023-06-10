#[allow(dead_code)]
pub fn run(str: &str) -> i32 {
    let mut count = 0;

    for c in str.chars() {
        if c == 'w' {
            count = count + 2;
        } else {
            count = count + 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(7, run("vvwvw"));
        assert_eq!(1, run("v"));
        assert_eq!(12, run("wwwvvvvvv"));
    }
}
