// https://atcoder.jp/contests/joi2006yo/tasks/joi2006yo_b

pub fn run(n: usize, dic: Vec<(char, char)>, q: usize, vec: Vec<char>) -> String {
    let before: Vec<char> = dic.iter().map(|t| t.0).collect();
    let after: Vec<char> = dic.iter().map(|t| t.1).collect();

    vec.iter()
        .map(|c| {
            if let Some(index) = before.iter().position(|char| char == c) {
                after[index]
            } else {
                *c
            }
        })
        .collect()
}

fn main() {
    println!("{}", run(3, vec![('A', 'a'), ('0', '5'), ('5', '4')], 10, vec!['A', 'B', 'C', '0', '1', '4', '5', 'a', 'b', 'A']));
}
