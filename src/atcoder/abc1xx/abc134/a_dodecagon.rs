// https://atcoder.jp/contests/abc134/tasks/abc134_a

pub fn run(r: usize) -> usize {
	3 * r * r
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(48, run(4));
		assert_eq!(675, run(15));
		assert_eq!(19200, run(80));
	}
}
