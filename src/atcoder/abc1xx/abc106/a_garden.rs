// https://atcoder.jp/contests/abc106/tasks/abc106_a

pub fn run(a: usize, b: usize) -> usize {
	(a - 1) * (b - 1)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(1, run(2, 2));
		assert_eq!(24, run(5, 7));
	}
}
