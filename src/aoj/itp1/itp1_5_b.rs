// https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/5/ITP1_5_B

fn run(hw: Vec<(usize, usize)>) -> Vec<Vec<String>> {
    let mut ans = Vec::new();

    for (h, w) in hw {
        if h == 0 {
            return ans;
        }

        let mut vec = Vec::new();

        for i in 0..h {
            let mut str = String::new();

            for j in 0..w {
                if i == 0 || i == h-1 {
                    str.push('#');
                } else if j == 0 || j == w-1 {
                    str.push('#');
                } else {
                    str.push('.');
                }
            }

            vec.push(str);
        }

        ans.push(vec);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(Vec<(usize, usize)>, Vec<Vec<&'static str>>);

    #[test]
    fn itp1_5_a() {
        let tests = [
            TestCase(vec![(3, 4), (5, 6), (3, 3), (0, 0)], vec![vec!["####", "#..#", "####"], vec!["######", "#....#", "#....#", "#....#", "######"], vec!["###", "#.#", "###"]]),
        ];

        for TestCase(hw, expected) in tests {
            assert_eq!(run(hw), expected);
        }
    }
}
