// https://atcoder.jp/contests/abc141/tasks/abc141_a

pub fn run(s: &str) -> String {
	match s {
		"Sunny" => String::from("Cloudy"),
		"Cloudy" => String::from("Rainy"),
		"Rainy" => String::from("Sunny"),
		_ => unreachable!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("Cloudy"), run("Sunny"));
		assert_eq!(String::from("Sunny"), run("Rainy"));
		assert_eq!(String::from("Rainy"), run("Cloudy"));
	}
}
