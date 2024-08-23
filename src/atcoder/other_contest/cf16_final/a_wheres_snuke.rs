// https://atcoder.jp/contests/cf16-final/tasks/codefestival_2016_final_a

pub fn run(_h: usize, _w: usize, s: Vec<Vec<&str>>) -> (char, usize) {
    for (i, vec) in s.into_iter().enumerate() {
        for (j, str) in vec.iter().enumerate() {
            if *str == "snuke" {
                let row = (b'A' + (j) as u8) as char;
                println!("{}", row);

                return (row, i+1);
            }
        }
    }

    unreachable!()
}
