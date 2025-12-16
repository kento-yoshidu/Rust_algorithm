// https://atcoder.jp/contests/abc201/tasks/abc201_c

fn run(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();

    let mut ans = 0;

    for i in 0..10000 {
        let str = format!("{:04}", i);

        let mut flags = vec![false; 10];

        for c in str.chars() {
            let i = c.to_digit(10).unwrap();

            flags[i as usize] = true;
        }

        let mut flag = true;

        for j in 0..10 {
            if chars[j] == 'o' && flags[j] == false {
                flag = false;
                break;
            }

            if chars[j] == 'x' && flags[j] {
                flag = false;
                break;
            }
        }

        if flag {
            ans += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc201_c() {
        let tests = [
            TestCase("ooo???xxxx", 108),
            TestCase("o?oo?oxoxo", 0),
            TestCase("xxxxx?xxxo", 15),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
