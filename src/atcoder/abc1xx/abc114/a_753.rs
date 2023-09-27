// https://atcoder.jp/contests/abc114/tasks/abc114_a

fn run(x: i32) -> String {
    if x == 3 || x == 5 || x == 7 {
        String::from("YES")
    } else {
        String::from("NO")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("YES"), run(5));
        assert_eq!(String::from("NO"), run(6));
    }
}
