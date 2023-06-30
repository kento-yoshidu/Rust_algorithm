#[allow(dead_code)]
pub fn test(s: String) -> usize {
    let c: Vec<char> = s.chars().collect();

    let ans = c.iter().filter(|c| {
        println!("{}", c);

        true
    }).count();

    ans
}
