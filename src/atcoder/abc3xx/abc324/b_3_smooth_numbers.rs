// https://atcoder.jp/contests/abc324/tasks/abc324_b

pub fn run(n: usize) -> String {
    let mut num = n;

    while num % 2 == 0 {
        num /= 2
    }

    while num % 3 == 0 {
        num /= 3;
    }

    if num == 1 {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(324));
        assert_eq!(String::from("No"), run(5));
        assert_eq!(String::from("Yes"), run(32));
        assert_eq!(String::from("Yes"), run(37748736));
    }
}
