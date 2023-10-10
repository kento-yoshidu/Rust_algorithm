// https://atcoder.jp/contests/abc284/tasks/abc284_a

pub fn run(_n: usize, s: Vec<&str>) -> Vec<&str> {
    s.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec!["Snuke", "Aoki", "Takahashi"], run(3, vec!["Takahashi", "Aoki", "Snuke"]));
        assert_eq!(vec!["Happy", "New", "Year", "2023"], run(4, vec!["2023", "Year", "New", "Happy"]));
    }
}
