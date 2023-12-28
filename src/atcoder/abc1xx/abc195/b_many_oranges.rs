// https://atcoder.jp/contests/abc195/tasks/abc195_b

pub fn run(a: usize, b: usize, w: usize) -> String {
    let weight = w*1000;

    let min = 0;
    let max = std::usize::MAX;

    for i in 1..=1000 * 1000 {
        let light = i*a;
        let heavy = i*b;

        if light <= weight && weight <= heavy {
            println!("min = {}, max = {}, minmax={}", light, heavy, light*heavy);
        }

    }

    String::from("UNSATISFIABLE")
}
