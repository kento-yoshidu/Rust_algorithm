// https://atcoder.jp/contests/abc177/tasks/abc177_b

pub fn run(s: String, t: String) -> usize {
    let mut ans = std::usize::MAX;

    let ss: Vec<char> = s.chars().collect();
    let tt: Vec<char> = t.chars().collect();

    for i in 0..=(ss.len() - tt.len()) {
        let mut differ_count = 0;

        for j in 0..tt.len() {
            if ss[i+j] != tt[j] {
                differ_count += 1;
            }
        }

        ans = ans.min(differ_count);
    }

    ans
}

/*
pub fn run2(s: &str, t: &str) -> usize {
    let s_vec: Vec<char> = s.chars().collect();
    let t_vec: Vec<char> = s.chars().collect();

    let t_len = t.len();

    (0..=t_len)
        .for_each(|num| {
            s_vec[num..t_len+num].iter()
                .zip(t_vec.iter())
                .for_each(|t| {
                    println!("{:?}", t);
                })
                //.count()
        //})
        //.min()
        //.unwrap()
});
10
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, run(String::from("cabacc"), String::from("abc")));
        assert_eq!(6, run(String::from("codeforces"), String::from("atcoder")));
    }

    /*
    #[test]
    fn test2() {
        assert_eq!(1, run2("cabacc", "abc"));
        assert_eq!(6, run2("codeforces", "atcoder"));
    }
    */
}
