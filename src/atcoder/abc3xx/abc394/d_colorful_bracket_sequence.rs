// https://atcoder.jp/contests/abc394/tasks/abc394_d

fn run(s: &str) -> &'static str {
    let str: String = s.chars()
        .fold(Vec::new(), |mut stack, c| {
            stack.push(c);

            let len = stack.len();

            match c {
                ')' => {
                    if len > 1 && stack[len-2] == '(' {
                        stack.truncate(len-2);
                    }
                },
                ']' => {
                    if len > 1 && stack[len-2] == '[' {
                        stack.truncate(len-2);
                    }
                },
                '>' => {
                    if len > 1 && stack[len-2] == '<' {
                        stack.truncate(len-2);
                    }
                },
                _ => (),
            }

            stack
        })
        .iter()
        .collect();

    if str.len() == 0 {
        "Yes"
    } else {
        "No"
    }
}

fn run2(s: &str) -> &'static str {
    let mut stack = Vec::new();

    for c in s.chars() {
        stack.push(c);

        if stack.len() > 1 {
            let len = stack.len();

            if stack.ends_with(&['(', ')']) ||
                stack.ends_with(&['[', ']']) ||
                stack.ends_with(&['<', '>'])
            {
                stack.truncate(len-2);
            }
        }
    }

    if stack.is_empty() {
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
    fn test() {
        let tests = [
            TestCase("([])<>()", "Yes"),
            TestCase("([<)]>", "No"),
            TestCase("())", "No"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
            assert_eq!(run2(s), expected);
        }
    }
}
