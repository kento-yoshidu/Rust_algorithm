// https://atcoder.jp/contests/abc019/tasks/abc019_2

fn run_length(s: Vec<char>) -> Vec<(char, usize)> {
    let mut result = vec![];
    let mut current = (s[0], 1);

    for i in 1..s.len() {
        if s[i] == current.0 {
            current.1 += 1;
        } else {
            result.push(current);
            current = (s[i], 1);
        }
    }

    result.push(current);

    result
}

pub fn run(s: &str) -> String {
    let rle = run_length(s.chars().collect());

    rle.iter()
        .map(|(c, i)| {
            format!("{}{}", c, i)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("a2b3a2d1"), run("aabbbaad"));
        assert_eq!(String::from("a2b12x1y1z1a1"), run("aabbbbbbbbbbbbxyza"));
        assert_eq!(String::from("e1d1c1b1a1"), run("edcba"));
    }
}
