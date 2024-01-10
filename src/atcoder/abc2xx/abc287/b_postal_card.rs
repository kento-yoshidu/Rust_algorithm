// https://atcoder.jp/contests/abc287/tasks/abc287_b

pub fn run(_n: usize, _m: usize, s: Vec<&str>, t: Vec<&str>) -> usize {
    let vec: Vec<&str> = s.iter().map(|str| &str[3..]).collect();

    vec.iter()
        .filter(|str| {
            t.iter()
                .any(|str2| {
                    *str == str2
                })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, run(3, 3, vec!["142857", "004159", "071028"], vec!["159", "287", "857"]));
        assert_eq!(3, run(5, 4, vec!["235983", "109467", "823476", "592801", "000333"], vec!["333", "108", "467", "983"]));
        assert_eq!(3, run(4, 4, vec!["000000", "123456", "987111", "000000"], vec!["000", "111", "999", "111"]));
    }
}
