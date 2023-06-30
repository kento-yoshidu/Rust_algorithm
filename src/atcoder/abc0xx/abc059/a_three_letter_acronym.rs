// https://atcoder.jp/contests/abc059/tasks/abc059_a

#[allow(dead_code)]
fn run(a: String, b: String, c: String) -> String {
    let vec = vec![a, b, c];

    let mut ans = String::new();

    vec.into_iter().for_each(|w| {
        ans.push(w.chars().nth(0).unwrap());
    });

    ans.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("ABC"), run(String::from("atcoder"), String::from("beginner"), String::from("contest")));
        assert_eq!(String::from("RRN"), run(String::from("resident"), String::from("register"), String::from("number")));
        assert_eq!(String::from("KNN"), run(String::from("k"), String::from("nearest"), String::from("neighbor")));
        assert_eq!(String::from("ALC"), run(String::from("async"), String::from("layered"), String::from("coding")));
    }
}
