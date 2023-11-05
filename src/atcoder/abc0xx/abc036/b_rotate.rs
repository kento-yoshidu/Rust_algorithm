// https://atcoder.jp/contests/abc036/tasks/abc036_b

pub fn run(n: usize, s: Vec<&str>) -> Vec<String> {
    let mut ans = vec![vec![' '; n]; n];

    s.iter()
        .enumerate()
        .for_each(|(i, str)| {
            str.chars()
                .enumerate()
                .for_each(|(j, c)| {
                    ans[j][n - 1 - i] = c
            })
        });

    ans.iter()
        .map(|chars| {
            chars.iter()
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![String::from("xxxo"), String::from("xxoo"), String::from("xxox"), String::from("xxxx")], run(4, vec!["ooxx", "xoox", "xxxx", "xxxx"]));
    }
}
