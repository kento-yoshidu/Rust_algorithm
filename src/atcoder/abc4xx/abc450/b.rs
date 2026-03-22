// https://atcoder.jp/contests/abc450/tasks/abc450_b

fn get(c: &Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    c[i][j - i - 1]
}

fn run(n: usize, c: Vec<Vec<usize>>) -> &'static str {
    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                if get(&c, i, j) + get(&c, j, k) < get(&c, i, k) {
                    return "Yes";
                }
            }
        }
    }

    "No"
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<Vec<usize>>, &'static str);

    #[test]
    fn abc450_b() {
        let tests = [
            TestCase(3, vec![vec![45, 450], vec![45]], "Yes"),
            TestCase(4, vec![vec![25, 40, 65], vec![30, 55], vec![25]], "No"),
        ];

        for TestCase(n, c, expected) in tests {
            assert_eq!(run(n, c), expected);
        }
    }
}
