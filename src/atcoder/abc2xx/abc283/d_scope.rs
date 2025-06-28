// https://atcoder.jp/contests/abc283/tasks/abc283_d

fn run(s: &str) -> &'static str {
    let s: Vec<char> = s.chars().collect();

    let mut is_stacked = vec![false; 26];

    let mut stack = Vec::new();

    for c in s {
        match c {
            '(' => {
                stack.push(c);
            },
            ')' => {
                while let Some(popped) = stack.pop() {
                    if popped == '(' {
                        break;
                    }
                    is_stacked[(popped as u8 - b'a') as usize] = false;
                }
            },
            _ => {
                if is_stacked[(c as u8 - b'a') as usize] {
                    return "No";
                }

                is_stacked[(c as u8 - b'a') as usize] = true;
                stack.push(c);
            }
        }
    }

    "Yes"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("((a)ba)", "Yes"),
            TestCase("(a(ba))", "No"),
            TestCase("(((())))", "Yes"),
            TestCase("abca", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
