// https://atcoder.jp/contests/abc136/tasks/abc136_a

pub fn run(a: usize, b: usize, c: usize) -> usize {
	let rest = a - b;

	if rest >= c {
		0
	} else {
		c - rest
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(1, run(6, 4, 3));
		assert_eq!(4, run(8, 3, 9));
		assert_eq!(0, run(12, 3, 7));
	}
}
