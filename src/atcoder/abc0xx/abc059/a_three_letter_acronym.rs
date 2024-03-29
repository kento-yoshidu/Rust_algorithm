// https://atcoder.jp/contests/abc059/tasks/abc059_a

fn run(a: String, b: String, c: String) -> String {
    let vec = vec![a, b, c];

    let mut ans = String::new();

    vec.into_iter().for_each(|w| {
        ans.push(w.chars().nth(0).unwrap());
    });

    ans.to_uppercase()
}

fn run2(a: String, b: String, c: String) -> String {
    let vec = vec![a, b, c];

    vec
        .iter()
        .map(|vec| {
            vec.chars().nth(0).unwrap()
        })
        .collect::<String>()
        .to_uppercase()
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

    #[test]
    fn test2() {
        assert_eq!(String::from("ABC"), run2(String::from("atcoder"), String::from("beginner"), String::from("contest")));
        assert_eq!(String::from("RRN"), run2(String::from("resident"), String::from("register"), String::from("number")));
        assert_eq!(String::from("KNN"), run2(String::from("k"), String::from("nearest"), String::from("neighbor")));
        assert_eq!(String::from("ALC"), run2(String::from("async"), String::from("layered"), String::from("coding")));
    }
}
