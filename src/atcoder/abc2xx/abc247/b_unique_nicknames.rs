// https://atcoder.jp/contests/abc247/tasks/abc247_b

pub fn run(_n: usize, vec: Vec<Vec<&str>>) -> String {
    for i in 0..(vec.len()) {
        let mut flag = true;

        // 🦀❓ ここおかしくない？
        for (j, _vv) in vec.iter().enumerate() {
            if i == j {
                continue;
            }

            if vec[i][0] != vec[j][0] && vec[i][0] != vec[j][1] && vec[i][1] != vec[j][0] && vec[i][1] != vec[j][1] {
                flag = false;
            }
        }

        if flag == true {
            return String::from("No");
        }
    }

    String::from("Yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("Yes"), run(3, vec![vec!["tanaka", "taro"], vec!["tanaka", "jiro"], vec!["suzuki", "hanako"]]));
        assert_eq!(String::from("No"), run(3, vec![vec!["aaa", "bbb"], vec!["xxx", "aaa"], vec!["bbb", "yyy"]]));
        assert_eq!(String::from("No"), run(2, vec![vec!["tanaka", "taro"], vec!["tanaka", "taro"]]));
        assert_eq!(String::from("Yes"), run(3, vec![vec!["takahashi", "chokudai"], vec!["aoki", "kensho"], vec!["snu", "ke"]]));
    }
}
