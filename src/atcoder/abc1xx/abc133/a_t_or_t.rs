// https://atcoder.jp/contests/abc133/tasks/abc133_a

pub fn run(n: usize, a: usize, b: usize) -> usize {
	b.min(n * a)
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(8, run(4, 2, 9));
		assert_eq!(7, run(4, 2, 7));
		assert_eq!(8, run(4, 2, 8));
	}
}
