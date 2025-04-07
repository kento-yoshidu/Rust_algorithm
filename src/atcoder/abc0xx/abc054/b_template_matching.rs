// https://atcoder.jp/contests/abc054/tasks/abc054_b

fn run(n: usize, m: usize, a_vec: Vec<&str>, b_vec: Vec<&str>) -> &'static str {
    let a_vec: Vec<Vec<char>> = a_vec.iter().map(|str| str.chars().collect()).collect();
    let b_vec: Vec<Vec<char>> = b_vec.iter().map(|str| str.chars().collect()).collect();

    let hw = if n % m == 0 { n / m } else { n / m + 1 };

    for i in 0..hw {
        for j in 0..hw {
            let mut vec = Vec::new();

            for k in 0..m {
                let mut v = Vec::new();

                for l in 0..m {
                    v.push(a_vec[i+k][j+l]);
                }

                vec.push(v);
            }

            if b_vec == vec {
                return "Yes"
            }
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, Vec<&'static str>, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, 2, vec!["#.#", ".#.", "#.#"], vec!["#.", ".#"], "Yes"),
            TestCase(4, 1, vec!["....", "....", "....", "...."], vec!["#"], "No"),
        ];

        for TestCase(n, m, a_vec, b_vec, expected) in tests {
            assert_eq!(expected, run(n, m, a_vec, b_vec));
        }
    }
}
