// https://atcoder.jp/contests/abc039/tasks/abc039_a

pub fn run(a: usize, b: usize, c: usize) -> usize {
	2 * (a*b + a*c + b*c)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(52, run(2, 3, 4));
		assert_eq!(52, run(3, 4, 2));
		assert_eq!(60000, run(100, 100, 100));
	}
}
