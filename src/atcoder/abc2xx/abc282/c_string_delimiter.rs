// https://atcoder.jp/contests/abc282/tasks/abc282_c

pub fn run(_n: usize, s: &str) -> String {
    let mut ans = String::new();

    let mut flag = true;

    for c in s.chars() {
        if c == '"' {
            flag = !flag;
            ans.push('"');
            continue;
        }

        if c == ',' {
            if flag == true {
                ans.push('.');
            } else {
                ans.push(',');
            }
        } else {
            ans.push(c);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(8, "\"a,b\"c,d", "\"a,b\"c.d"),
            TestCase(5, ",,,,,", "....."),
            TestCase(20, "a,\"t,\"c,\"o,\"d,\"e,\"r,", "a.\"t,\"c.\"o,\"d.\"e,\"r."),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }

    }
}
