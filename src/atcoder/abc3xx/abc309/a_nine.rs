// https://atcoder.jp/contests/abc309/tasks/abc309_a

fn run(a: usize, b: usize) -> String {
    if a % 3 == 0 {
        String::from("No")
    } else if b - a != 1 {
        String::from("No")
    } else {
        String::from("Yes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(7, 8));
        assert_eq!(String::from("No"), run(1, 9));
        assert_eq!(String::from("No"), run(3, 4));
    }
}
