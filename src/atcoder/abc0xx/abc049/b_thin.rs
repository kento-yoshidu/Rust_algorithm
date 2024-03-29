// https://atcoder.jp/contests/abc049/tasks/abc049_b

pub fn run(_h: usize, _w: usize, c: Vec<&str>) -> Vec<String> {
    c.iter().map(|r| {
        format!("{}", r).repeat(2)
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![String::from("*.*."), String::from(".*.*")], run(2, 2, vec!["*.", ".*"]));
        assert_eq!(vec![String::from("***.***.")], run(1, 4, vec!["***."]));
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
    }
}
