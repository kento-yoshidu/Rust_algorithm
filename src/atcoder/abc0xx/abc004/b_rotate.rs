// https://atcoder.jp/contests/abc004/tasks/abc004_2

pub fn run(s: Vec<&str>) -> Vec<String> {
    let mut ans = Vec::<String>::new();

    for v in s.iter().rev() {
        let mut row = String::new();

        for c in v.chars().rev() {
            row.push(c);
        }

        ans.push(row);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![String::from("...."),
                        String::from(".xx."),
                        String::from(".oo."),
                        String::from("....")],
                            run(vec!["....",
                                     ".oo.",
                                     ".xx.",
                                     "...."]));
        assert_eq!(vec![String::from("ooxx"),
                        String::from("ooxx"),
                        String::from("xxoo"),
                        String::from("xxoo")],
                            run(vec!["ooxx",
                                     "ooxx",
                                     "xxoo",
                                     "xxoo"]));
        assert_eq!(vec![String::from("o..."),
                        String::from("...."),
                        String::from("...."),
                        String::from("....")],
                            run(vec!["....",
                                     "....",
                                     "....",
                                     "...o"]));
    }
}
