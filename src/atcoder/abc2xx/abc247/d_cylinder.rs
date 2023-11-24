/*
#[allow(dead_code, unused)]
fn run_lengths(vec: Vec<&Vec<usize>>) -> Vec<(usize, usize)> {
    let clone_vec = vec.clone();

    let mut run_lengths = vec![(0, 0)];

    for mut v in vec.iter() {
        if v[1] == run_lengths.last().unwrap().0 {
            run_lengths = (v[1], run_lengths.last().unwrap().1 + v[2]);
        } else {
            run_lengths.push((v[1], v[2]));
        }
    }

    run_lengths
}

#[allow(unused)]
pub fn run(n: usize, q: Vec<Vec<usize>>) -> usize {
    let mut ans: Vec<usize> = Vec::new();

    let vec_a: Vec<_> = q.iter().filter(|arr| arr[0] == 1).collect();
    let vec_b: Vec<_> = q.iter().filter(|arr| arr[0] == 2).collect();

    let r_l = run_lengths(vec_a);

    10

}

fn main() {
    println!("{}", run(4, vec![vec![1, 2, 3], vec![2, 2], vec![1, 3, 4], vec![2, 3], vec![1, 3, 10]]));
}
*/
