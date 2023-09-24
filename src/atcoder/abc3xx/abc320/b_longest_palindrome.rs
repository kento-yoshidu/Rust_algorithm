// https://atcoder.jp/contests/abc320/tasks/abc320_a

fn check(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

pub fn run(s: String) -> usize {
    let mut ans = 1;

    for i in 0..s.len() {
        for j in i+1..s.len() {
            if check(&s[i..=j]) {
                ans = ans.max(j+1 - i);
            }
        }
    }

    ans
}

fn main() {
    println!("{}", run(String::from("TOYOTA")));
    println!("{}", run(String::from("ABCDEFG")));
    println!("{}", run(String::from("AAAAAAAAAA")));
    println!("{}", run(String::from("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")));
}
