// https://atcoder.jp/contests/abc079/tasks/abc079_a

fn run(n: String) -> String {
	let mut vec: Vec<char> = n.chars().collect();

	vec.dedup();

	if vec.len() <= 2 {
		String::from("Yes")
	} else {
		String::from("No")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("Yes"), run(String::from("1118")));
		assert_eq!(String::from("No"), run(String::from("1181")));
		assert_eq!(String::from("Yes"), run(String::from("7777")));
		assert_eq!(String::from("No"), run(String::from("1234")));
	}
}
