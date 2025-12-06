// https://atcoder.jp/contests/abc237/tasks/abc237_c

fn check(s: String) -> bool {
    s.chars().eq(s.chars().rev())
}

fn run(s: &str) -> &'static str {
    // 先頭、末尾から連続して何文字aが続くかをカウント
    let mut head = 0;
    let mut tail = 0;

    for c in s.chars() {
        if c == 'a' {
            head += 1;
        } else {
            break
        }
    }

    for c in s.chars().rev() {
        if c == 'a' {
            tail += 1;
        } else {
            break
        }
    }

    // 先頭のaの方が多い時
    if head > tail {
        return "No"
    }

    // 全てaの時
    if head == s.len() {
        return "Yes"
    }

    let mut vec: Vec<char> = s.chars().collect();

    // 先頭と末尾の連続するaを削除
    vec.drain(0..head);
    vec.drain((vec.len()-tail)..vec.len());

    let str: String = vec.into_iter().collect();

    if check(str) {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc237_c() {
        let tests = [
            TestCase("kasaka", "Yes"),
            TestCase("atcoder", "No"),
            TestCase("php", "Yes"),
            TestCase("aaaaaaaa", "Yes"),
            TestCase("aaabaaa", "Yes"),
            TestCase("aaaabaaa", "No"),
            TestCase("aaabaaaa", "Yes"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
