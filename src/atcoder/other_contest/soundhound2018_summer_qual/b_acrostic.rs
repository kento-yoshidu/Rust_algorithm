// https://atcoder.jp/contests/soundhound2018-summer-qual/tasks/soundhound2018_summer_qual_b

fn run(s: &str, w: usize) -> String {
    s.chars()
        .step_by(w)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run("abcdefgh", 3), "adg");
        assert_eq!(run("11111", 1), "11111");
        assert_eq!(run("souuundhound", 2), "suudon");
    }
}
