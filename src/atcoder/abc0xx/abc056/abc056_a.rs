fn run(a: &str, b: &str) -> String {
    if a == "H" {
        return b.to_string()
    }

    if b == "H" {
        "D".to_string()
    } else {
        "H".to_string()
    }
}

fn main() {
    println!("{:?}", "H".to_string() == run("H", "H"));
    println!("{:?}", "D".to_string() == run("D", "H"));
    println!("{:?}", "H".to_string() == run("D", "D"));
}
