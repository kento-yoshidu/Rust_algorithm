#[allow(dead_code)]
pub fn run(a: Vec<i32>, b: Vec<i32>) -> Vec<&'static str> {
    let mut ans: Vec<&str> = Vec::new();

    for num in b.iter() {
        let mut flag = false;

        for i in 0..1 << a.len() {
            // bit探索ごとの合計値
            let mut sum = 0;

            for j in 0..a.len() {
                if (1 << j) & i == 0 {
                    sum += a[j];
                }
            }

            if sum == *num as i32 {
                flag = true
            }
        }

        if flag == true {
            ans.push("yes")
        } else {
            ans.push("no")
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec!["no", "no", "yes", "yes"], run(vec![1, 5, 7, 10, 21], vec![2, 4, 17, 8]));
    }
}
