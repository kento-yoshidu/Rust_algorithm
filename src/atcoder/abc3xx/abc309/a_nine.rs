// https://atcoder.jp/contests/abc309/tasks/abc309_a

fn run(s: [usize; 8]) -> String {
    for i in s {
        if !(100 <= i && i <= 675) || i % 25 != 0 {
            return String::from("No");
        }
    }

    for v in s.windows(2) {
        if v[0] > v[1] {
            return String::from("No")
        }
    }

    String::from("Yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run([125, 175, 250, 300, 400, 525, 600, 650]));
        assert_eq!(String::from("No"), run([100, 250, 300, 400, 325, 575, 625, 675]));
        assert_eq!(String::from("No"), run([0, 23, 24, 145, 301, 413, 631, 632]));
    }
}
