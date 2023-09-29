// https://atcoder.jp/contests/abc085/tasks/abc085_a

fn run(s: String) -> String {
	let vec: Vec<&str> = s.split("/").collect();

	format!("2018/{}/{}", vec[1], vec[2])
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("2018/01/07"), run(String::from("2017/01/07")));
		assert_eq!(String::from("2018/01/31"), run(String::from("2017/01/31")));
	}
}
