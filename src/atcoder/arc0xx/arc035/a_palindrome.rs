// https://atcoder.jp/contests/arc035/tasks/arc035_a

pub fn run(s: &str) -> &'static str {
    let len = s.len();

    let chars: Vec<char> = s.chars().collect();

    for i in 0..len/2 {
        println!("{}, {}", chars[i], chars[len-i-1]);

        if chars[i] != '*' && chars[len-i-1] != '*' && chars[i] != chars[len-i-1] {
            return "NO";
        }
    }

    "YES"
}
