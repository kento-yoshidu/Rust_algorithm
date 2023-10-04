// https://atcoder.jp/contests/abc071/tasks/abc071_a

pub fn run(x: isize, a: isize, b: isize) -> String {
	let abs_a = (x - a).abs();
	let abs_b = (x - b).abs();

	if abs_a < abs_b {
		String::from("A")
	} else {
		String::from("B")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(String::from("B"), run(5, 2, 7));
		assert_eq!(String::from("A"), run(1, 999, 1000));
	}
}
