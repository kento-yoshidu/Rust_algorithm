// https://atcoder.jp/contests/abc346/tasks/abc346_b

pub fn run(w: usize, b: usize) -> &'static str {
    let str = "wbwbwwbwbwbw".repeat(w+b);
    let vec: Vec<char> = str.chars().collect();

    for s in vec.windows(w+b) {
        let w_count = s.iter().filter(|c| **c == 'w').count();
        let b_count = s.iter().filter(|c| **c == 'b').count();

        if w_count == w && b_count == b {
            return "Yes"
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, "Yes"),
            TestCase(3, 0, "No"),
            TestCase(92, 66, "Yes"),
        ];

        for TestCase(w, b, expected) in tests {
            assert_eq!(run(w, b), expected);
        }
    }
}
