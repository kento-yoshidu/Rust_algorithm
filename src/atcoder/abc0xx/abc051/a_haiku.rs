// https://atcoder.jp/contests/abc051/tasks/abc051_a

pub fn run(s: String) -> String {
    s.replace(",", " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("happy newyear enjoy", run(String::from("happy,newyear,enjoy")));
        assert_eq!("haiku atcoder tasks", run(String::from("haiku,atcoder,tasks")));
        assert_eq!("abcde fghihgf edcba", run(String::from("abcde,fghihgf,edcba")));
    }
}
