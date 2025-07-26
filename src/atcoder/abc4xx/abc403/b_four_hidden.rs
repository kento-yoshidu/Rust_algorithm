// https://atcoder.jp/contests/abc403/tasks/abc403_b

fn run(t: &str, u: &str) -> &'static str {
    let t: Vec<char> = t.chars().collect();
    let u: Vec<char> = u.chars().collect();

    for i in 0..(t.len() - u.len()+1) {
        let mut flag = true;

        for j in 0..u.len() {
            if t[i+j] != u[j] && t[i+j] != '?' {
                flag = false;
            }
        }

        if flag {
            return "Yes";
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("tak??a?h?", "nashi", "Yes"),
            TestCase("??e??e", "snuke", "No"),
            TestCase("????", "aoki", "Yes"),
        ];

        for TestCase(t, u, expected) in tests {
            assert_eq!(run(t, u), expected);
        }
    }
}
