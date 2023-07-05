#[allow(dead_code)]
pub fn run(vec: Vec<Vec<i32>>) -> Vec<&'static str> {
    let mut ans: Vec<&str> = Vec::new();

    for v in vec {
        if v[0] == -1 || v[1] == -1 {
            ans.push("F");
        } else if v[0] + v[1] >= 80 {
            ans.push("A");
        } else if v[0] + v[1] >= 65 {
            ans.push("B")
        } else if v[0] + v[1] >= 50 {
            ans.push("C")
        } else if v[0] + v[1] >= 30 {
            if v[2] >= 50 {
                ans.push("C")
            } else {
                ans.push("D")
            }
        } else {
            ans.push("F")
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec!["A", "C", "F"], run(vec![vec![40, 42, -1], vec![20, 30, -1], vec![0, 2, -1]]));
    }
}

// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_7_A
