// https://atcoder.jp/contests/abc075/tasks/abc075_a

fn run(a: isize, b: isize, c: isize) -> isize {
	a ^ b ^ c
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(7, run(5, 7, 5));
		assert_eq!(7, run(1, 1, 7));
		assert_eq!(-100, run(-100, 100, 100));
	}
}
