// https://atcoder.jp/contests/abc135/tasks/abc135_a

pub fn run(a: usize, b: usize) -> String {
	if (a + b) % 2 == 0 {
		((a + b) / 2).to_string()
	} else {
		String::from("IMPOSSIBLE")
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("9"), run(2, 16));
		assert_eq!(String::from("IMPOSSIBLE"), run(0, 3));
		assert_eq!(String::from("549034394"), run(998244353, 99824435));
	}
}
