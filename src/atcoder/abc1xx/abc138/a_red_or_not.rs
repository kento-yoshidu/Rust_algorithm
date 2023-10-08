// https://atcoder.jp/contests/abc138/tasks/abc138_a

pub fn run(a: usize, s: String) -> String {
	if a >= 3200 {
		s
	} else {
		String::from("red")
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("pink"), run(3200, String::from("pink")));
		assert_eq!(String::from("red"), run(3199, String::from("pink")));
		assert_eq!(String::from("red"), run(4049, String::from("red")));
	}
}
