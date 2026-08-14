
use crate::{Decode, Error, Input};

const DECODE_MAX_DEPTH_MSG: &str = "Maximum recursion depth reached when decoding";

pub trait DecodeLimit: Sized {
	
	fn decode_with_depth_limit<I: Input>(limit: u32, input: &mut I) -> Result<Self, Error>;

	fn decode_all_with_depth_limit(limit: u32, input: &mut &[u8]) -> Result<Self, Error>;
}

struct DepthTrackingInput<'a, I> {
	input: &'a mut I,
	depth: u32,
	max_depth: u32,
}

impl<I: Input> Input for DepthTrackingInput<'_, I> {
	fn remaining_len(&mut self) -> Result<Option<usize>, Error> { panic!("STUB: not implemented") }

	fn read(&mut self, into: &mut [u8]) -> Result<(), Error> { panic!("STUB: not implemented") }

	fn read_byte(&mut self) -> Result<u8, Error> { panic!("STUB: not implemented") }

	fn descend_ref(&mut self) -> Result<(), Error> { panic!("STUB: not implemented") }

	fn ascend_ref(&mut self) { panic!("STUB: not implemented") }

	fn on_before_alloc_mem(&mut self, size: usize) -> Result<(), Error> { panic!("STUB: not implemented") }
}

impl<T: Decode> DecodeLimit for T {
	fn decode_all_with_depth_limit(limit: u32, input: &mut &[u8]) -> Result<Self, Error> { panic!("STUB: not implemented") }

	fn decode_with_depth_limit<I: Input>(limit: u32, input: &mut I) -> Result<Self, Error> { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Encode;

	#[test]
	fn decode_limit_works() {
		type NestedVec = Vec<Vec<Vec<Vec<u8>>>>;
		let nested: NestedVec = vec![vec![vec![vec![1]]]];
		let encoded = nested.encode();

		let decoded = NestedVec::decode_with_depth_limit(3, &mut encoded.as_slice()).unwrap();
		assert_eq!(decoded, nested);
		assert!(NestedVec::decode_with_depth_limit(2, &mut encoded.as_slice()).is_err());
	}

	#[test]
	fn decode_limit_advances_input() {
		type NestedVec = Vec<Vec<Vec<Vec<u8>>>>;
		let nested: NestedVec = vec![vec![vec![vec![1]]]];
		let encoded = nested.encode();
		let encoded_slice = &mut encoded.as_slice();

		let decoded = Vec::<u8>::decode_with_depth_limit(1, encoded_slice).unwrap();
		assert_eq!(decoded, vec![4]);
		assert!(NestedVec::decode_with_depth_limit(3, encoded_slice).is_err());
	}

	#[test]
	fn decode_all_with_limit_advances_input() {
		type NestedVec = Vec<Vec<Vec<Vec<u8>>>>;
		let nested: NestedVec = vec![vec![vec![vec![1]]]];
		let mut encoded = NestedVec::encode(&nested);

		let decoded = NestedVec::decode_all_with_depth_limit(3, &mut encoded.as_slice()).unwrap();
		assert_eq!(decoded, nested);

		encoded.extend(&[1, 2, 3, 4, 5, 6]);
		assert_eq!(
			NestedVec::decode_all_with_depth_limit(3, &mut encoded.as_slice())
				.unwrap_err()
				.to_string(),
			"Input buffer has still data left after decoding!",
		);
	}
}
