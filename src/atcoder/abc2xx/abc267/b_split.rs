// https://atcoder.jp/contests/abc267/tasks/abc267_b

pub fn run(s: &str) -> &'static str {
    let s: Vec<char> = s.chars().collect();

    if s[0] == '1' {
        return "No";
    }

    let mut vec = vec![0; 7];

    for (i, c) in s.iter().enumerate() {
        if *c == '0' {
            match i {
                6 => vec[0] += 1,
                3 => vec[1] += 1,
                1 | 7 => vec[2] += 1,
                0 | 4 => vec[3] += 1,
                2 | 8 => vec[4] += 1,
                5 => vec[5] += 1,
                9 => vec[6] += 1,
                _ => unreachable!(),
            }
        }
    }

    // 全て倒れていたらtrue、1本でも残っていたらfalse
    let vec: Vec<bool> = vec.iter()
        .enumerate()
        .map(|(i, num)| {
            match i {
                0 | 1 | 5 | 6 => {
                    if *num == 1 {
                        true
                    } else {
                        false
                    }
                },
                2 | 3 | 4 => {
                    if *num == 2 {
                        true
                    } else {
                        false
                    }
                },
                _ => unreachable!(),
            }
        })
        .collect();

    for i in 0..7 {
        for j in i+2..7 {
            if vec[i] == false && vec[j] == false {
                for k in i+1..j {
                    if vec[k] == true {
                        return "Yes";
                    }
                }
            }
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase("0101110101", "Yes"),
            TestCase("0100101001", "Yes"),
            TestCase("0000100110", "No"),
            TestCase("1101110101", "No"),
            TestCase("0000001001", "Yes"),
            TestCase("1000000000", "No"),
            TestCase("0000000000", "No"),
            TestCase("0100001011", "Yes"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
