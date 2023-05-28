pub fn run(x: u16, a: u16, b: u16) -> u16 {
	let b_count = (x - a) / b;

	x - a - b * b_count
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(84, run(1234, 150, 100));
		assert_eq!(28, run(1000, 108, 108));
		assert_eq!(0, run(579, 123, 456));
		assert_eq!(405, run(7477, 549, 593));
	}
}
