// https://atcoder.jp/contests/abc115/tasks/abc115_a

fn run(x: i32) -> String {
    match x {
        25 => String::from("Christmas"),
        24 => String::from("Christmas Eve"),
        23 => String::from("Christmas Eve Eve"),
        _ => String::from("Christmas Eve Eve Eve"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("Christmas", run(25));
        assert_eq!("Christmas Eve Eve Eve", run(22));
    }
}
