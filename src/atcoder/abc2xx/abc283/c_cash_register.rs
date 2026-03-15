// https://atcoder.jp/contests/abc283/tasks/abc283_c

fn run(s: &str) -> usize {
    let mut vec: Vec<char> = s.chars().collect();

    let mut count = 0;

    for i in 0..(vec.len()-1) {
        if vec[i] == '0' && vec[i+1] == '0' {
            vec[i+1] = 'x';
            count += 1;
        }
    }

    vec.len() - count
}

fn run2(s: &str) -> usize {
    let mut vec: String = s.chars().collect();

    vec = vec.replace("00", "x");

    vec.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc283_c() {
        let tests = [
            TestCase("40004", 4),
            TestCase("1355506027", 10),
            TestCase("10888869450418352160768000001", 27),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
