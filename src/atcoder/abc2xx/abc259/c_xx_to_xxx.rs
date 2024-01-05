// https://atcoder.jp/contests/abc259/tasks/abc259_c

fn run_lengths(s: Vec<char>) -> Vec<(char, usize)> {
    let mut i = 0;
    let mut run_lengths = vec![];
    let mut current = (s[0], 0);

    loop {
        while i < s.len() && s[i] == current.0 {
            current.1 += 1;
            i += 1;
        }

        run_lengths.push(current);

        if i == s.len() {
            break;
        } else {
            current = (s[i], 0);
        }
    }

    run_lengths
}

pub fn run(s: &str, t: &str) -> String {
    let s_length = run_lengths(s.chars().collect());
    let t_length = run_lengths(t.chars().collect());

    if s_length.iter()
        .zip(t_length.iter())
        .all(|(a, b)| {
            a.0 == b.0 && (a.1 == b.1 || (a.1 < b.1 && a.1 > 1))
        }) {
            String::from("Yes")
        } else {
            String::from("No")
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run("abbaac", "abbbbaaac"));
        assert_eq!(String::from("No"), run("xyzz", "xyyzz"));
        assert_eq!(String::from("Yes"), run("aa", "aa"));
    }
}
