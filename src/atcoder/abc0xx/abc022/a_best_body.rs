#[allow(unused)]
pub fn run(n: usize, s: usize, t: usize, w: Vec<usize>) -> usize {
    let vec = w.iter().fold(0, |sum, x| {
        sum + x
    });

    println!("{:?}", vec);

    10
}
