// https://atcoder.jp/contests/abc267/tasks/abc267_a

pub fn run(s: String) -> usize {
    let arr = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

    arr.iter().rev().position(|day| **day == s).unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(String::from("Friday")));
        assert_eq!(2, run(String::from("Thursday")));
        assert_eq!(3, run(String::from("Wednesday")));
        assert_eq!(4, run(String::from("Tuesday")));
        assert_eq!(5, run(String::from("Monday")));
    }
}
