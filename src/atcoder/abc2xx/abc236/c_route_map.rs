// https://atcoder.jp/contests/abc236/tasks/abc236_c

pub fn run(_n: usize, _m: usize, s: Vec<&str>, t: Vec<&str>) -> Vec<String> {
    s.iter()
        .map(|station| {
            if t.contains(station) {
                String::from("Yes")
            } else {
                String::from("No")
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![String::from("Yes"), String::from("No"), String::from("Yes"), String::from("No"), String::from("Yes")], run(5, 3, vec!["tokyo", "kanda", "akiba", "okachi", "ueno"], vec!["tokyo", "akiba", "ueno"]));
        assert_eq!(vec![String::from("Yes"), String::from("Yes"), String::from("Yes"), String::from("Yes"), String::from("Yes"), String::from("Yes"), String::from("Yes")], run(7, 7, vec!["a", "t", "c", "o", "d", "e", "r"], vec!["a", "t", "c", "o", "d", "e", "r"]));
    }
}
