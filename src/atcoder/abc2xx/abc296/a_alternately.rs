// https://atcoder.jp/contests/abc296/tasks/abc296_a

pub fn run(n: usize, str: &str) -> &str {
    for i in 0..n - 1 {
        if str.chars().nth(i).unwrap() == str.chars().nth(i + 1).unwrap() {
            return "No";
        }
    }

    "Yes"
}

pub fn run2(_n: usize, str: &str) -> String {
    if str.chars()
        .collect::<Vec<char>>()
        .windows(2)
        .all(|v| { v[0] != v[1] }
    ) {
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
        assert_eq!("Yes", run(6, "MFMFMF"));
        assert_eq!("No", run(9, "FMFMMFMFM"));
        assert_eq!("Yes", run(1, "F"));
    }

    #[test]
    fn test2() {
        assert_eq!("Yes", run2(6, "MFMFMF"));
        assert_eq!("No", run2(9, "FMFMMFMFM"));
        assert_eq!("Yes", run2(1, "F"));
    }
}
