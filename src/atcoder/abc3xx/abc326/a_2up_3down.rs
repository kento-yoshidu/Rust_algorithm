// https://atcoder.jp/contests/abc326/tasks/abc326_a

pub fn run(x: i32, y: i32) -> String {
    let dis = x - y;

    if dis.abs() > 3 {
        String::from("No")
    } else if dis < 0 && dis.abs() == 3 {
        String::from("No")
    } else {
        String::from("Yes")
    }
}
