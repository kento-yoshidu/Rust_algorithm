#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run(input: &str) {
    let v: Vec<&str> = input.split(' ').collect();

    let mut n: i32 = v[0].parse().unwrap();
    let k: i32 = v[1].parse().unwrap();

    for i in 1..=k {
        if n % 200 == 0 {
            n = n / 200;
        } else {
            // 整数Nを、Nの後ろに文字列として200 を付け加えた整数に置き換える。
            // 例えば、 7 を置き換えると 7200 に、 1234 を置き換えると 1234200 になります。
        }
    }

    println!("{}", n);
}
