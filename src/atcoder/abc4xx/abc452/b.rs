// https://atcoder.jp/contests/abc452/tasks/abc452_b

fn run(h: usize, w: usize) -> Vec<String> {
    let mut ans = Vec::new();

    for i in 0..h {
        if i == 0 || i == h-1 {
            ans.push("#".to_string().repeat(w));
            continue;
        }

        let mut str = String::new();

        for j in 0..w {
            if j == 0 || j == w-1 {
                str.push('#');
            } else {
                str.push('.');
            }
        }

        ans.push(str);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>);

    #[test]
    fn abc452_b() {
        let tests = [
            TestCase(4, 5, vec!["#####", "#...#", "#...#", "#####"]),
            TestCase(5, 6, vec!["######", "#....#", "#....#", "#....#", "######"]),
        ];

        for TestCase(h, w, expected) in tests {
            assert_eq!(run(h, w), expected);
        }
    }
}
