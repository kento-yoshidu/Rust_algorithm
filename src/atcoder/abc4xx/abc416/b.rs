// https://atcoder.jp/contests/abc416/tasks/abc416_b

fn run(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();

    let mut ans = String::new();

    let mut flag = true;

    for c in chars {
        match c {
            '#' => {
                ans.push(c);
                flag = true;
            },
            '.' => {
                if flag {
                    ans.push('o');
                    flag = false;
                } else {
                    ans.push(c);
                }
            },
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc416_b() {
        let tests = [
            TestCase("#..#.", "#o.#o"),
            TestCase("#", "#"),
            TestCase(".....", "o...."),
            TestCase("...#..#.##.#.", "o..#o.#o##o#o"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
