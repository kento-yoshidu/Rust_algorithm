// https://atcoder.jp/contests/joi2023yo1b/tasks/joi2023_yo1b_d

pub fn run(n: usize, a: Vec<usize>, m: usize, b: Vec<usize>) -> usize {
    a.iter().fold(0, |mut score, num| {
        score += num;

        if b.contains(&score) {
            0
        } else {
            score
        }
    })
}

fn main() {
    println!("{}", run(4, vec![3, 1, 4, 1], 4, vec![2, 7, 1, 8]));
}
