
pub struct CountedInput<'a, I: crate::Input> {
	input: &'a mut I,
	counter: u64,
}

impl<'a, I: crate::Input> CountedInput<'a, I> {
	
	pub fn new(input: &'a mut I) -> Self { panic!("STUB: not implemented") }

	pub fn count(&self) -> u64 { panic!("STUB: not implemented") }
}

impl<I: crate::Input> crate::Input for CountedInput<'_, I> {
	fn remaining_len(&mut self) -> Result<Option<usize>, crate::Error> { panic!("STUB: not implemented") }

	fn read(&mut self, into: &mut [u8]) -> Result<(), crate::Error> { panic!("STUB: not implemented") }

	fn read_byte(&mut self) -> Result<u8, crate::Error> { panic!("STUB: not implemented") }

	fn ascend_ref(&mut self) { panic!("STUB: not implemented") }

	fn descend_ref(&mut self) -> Result<(), crate::Error> { panic!("STUB: not implemented") }

	fn on_before_alloc_mem(&mut self, size: usize) -> Result<(), crate::Error> { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::Input;

	#[test]
	fn test_counted_input_input_impl() {
		let mut input = &[1u8, 2, 3, 4, 5][..];
		let mut counted_input = CountedInput::new(&mut input);

		assert_eq!(counted_input.remaining_len().unwrap(), Some(5));
		assert_eq!(counted_input.count(), 0);

		counted_input.read_byte().unwrap();

		assert_eq!(counted_input.remaining_len().unwrap(), Some(4));
		assert_eq!(counted_input.count(), 1);

		counted_input.read(&mut [0u8; 2][..]).unwrap();

		assert_eq!(counted_input.remaining_len().unwrap(), Some(2));
		assert_eq!(counted_input.count(), 3);

		counted_input.ascend_ref();
		counted_input.descend_ref().unwrap();

		counted_input.read(&mut [0u8; 2][..]).unwrap();

		assert_eq!(counted_input.remaining_len().unwrap(), Some(0));
		assert_eq!(counted_input.count(), 5);

		assert_eq!(counted_input.read_byte(), Err("Not enough data to fill buffer".into()));

		assert_eq!(counted_input.remaining_len().unwrap(), Some(0));
		assert_eq!(counted_input.count(), 5);

		assert_eq!(
			counted_input.read(&mut [0u8; 2][..]),
			Err("Not enough data to fill buffer".into())
		);

		assert_eq!(counted_input.remaining_len().unwrap(), Some(0));
		assert_eq!(counted_input.count(), 5);
	}

	#[test]
	fn test_counted_input_max_count_read_byte() {
		let max_exact_count = u64::MAX - 1;

		let mut input = &[0u8; 1000][..];
		let mut counted_input = CountedInput::new(&mut input);

		counted_input.counter = max_exact_count;

		assert_eq!(counted_input.count(), max_exact_count);

		counted_input.read_byte().unwrap();

		assert_eq!(counted_input.count(), u64::MAX);

		counted_input.read_byte().unwrap();

		assert_eq!(counted_input.count(), u64::MAX);
	}

	#[test]
	fn test_counted_input_max_count_read() {
		let max_exact_count = u64::MAX - 1;

		let mut input = &[0u8; 1000][..];
		let mut counted_input = CountedInput::new(&mut input);

		counted_input.counter = max_exact_count;

		assert_eq!(counted_input.count(), max_exact_count);

		counted_input.read(&mut [0u8; 2][..]).unwrap();

		assert_eq!(counted_input.count(), u64::MAX);

		counted_input.read(&mut [0u8; 2][..]).unwrap();

		assert_eq!(counted_input.count(), u64::MAX);
	}
}
