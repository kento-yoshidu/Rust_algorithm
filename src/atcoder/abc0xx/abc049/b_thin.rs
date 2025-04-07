// https://atcoder.jp/contests/abc049/tasks/abc049_b

fn run(_h: usize, _w: usize, c: Vec<&str>) -> Vec<String> {
    c.iter().map(|r| {
        format!("{}", r).repeat(2)
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use std::usize;

    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(2, 2, vec!["*.", ".*"], vec!["*.*.", ".*.*"]),
            TestCase(1, 4, vec!["***."], vec!["***.***."]),
        /*
        assert_eq!(vec![String::from(".....***....***.........***....***......"),
                        String::from("....*...*..*...*........*...*..*...*...."),
                        String::from("...*.....**.....*......*.....*......*..."),
                        String::from("....*.....*....*.........**..*...**....."),
                        String::from(".......*..*.*...............**.*........"),
                        String::from(".........**............*.....**.....*..."),
                        String::from("...*.....*......*.......*.....*....*...."),
                        String::from(".....**..*...**............*..*.*......."),
                        String::from("........**.*.................**.........")],
                    run(9, 20, vec![".....***....***.....",
                                    "....*...*..*...*....",
                                    "...*.....**.....*...",
                                    "...*.....*......*...",
                                    "....*.....*....*....",
                                    ".....**..*...**.....",
                                    ".......*..*.*.......",
                                    "........**.*........",
                                    ".........**........."]));
        */
        ];

        for TestCase(n, w, c, expected) in tests {
            assert_eq!(run(n, w, c), expected);
        }
    }
}
