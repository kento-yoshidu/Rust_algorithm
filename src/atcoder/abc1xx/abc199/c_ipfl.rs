// https://atcoder.jp/contests/abc199/tasks/abc199_c

fn run(n: usize, s: &str, _q: usize, tab: Vec<(usize, usize, usize)>) -> String {
    let mut str: Vec<char> = s.chars().collect();

    let mut flag = false;

    for (t, a, b) in tab.into_iter() {
        match t {
            1 => {
                if flag == true {
                    let l = if a <= n {
                        n + a
                    } else {
                        a - n
                    };

                    let r = if b <= n {
                        n + b
                    } else {
                        b - n
                    };

                    str.swap(l-1, r-1);
                } else {
                    str.swap(a-1, b-1);
                }
            },
            2 => {
                flag = !flag;
            },
            _ => unreachable!(),
        }
    }

    let ans = if flag {
        [str[n..].to_vec(), str[..n].to_vec()].concat()
    } else {
        str
    };

    ans.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize, Vec<(usize, usize, usize)>, &'static str);

    #[test]
    fn abc199_c() {
        let tests = [
            TestCase(2, "FLIP", 2, vec![(2, 0, 0), (1, 1, 4)], "LPFI"),
            TestCase(2, "FLIP", 6, vec![(1, 1, 3), (2, 0, 0), (1, 1, 2), (1, 2, 3), (2, 0, 0), (1, 1, 4)], "ILPF"),
        ];

        for TestCase(n, s, q, tab, expected) in tests {
            assert_eq!(run(n, s, q, tab), expected);
        }
    }
}
