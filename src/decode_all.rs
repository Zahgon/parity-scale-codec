
use crate::{Decode, Error};

pub(crate) const DECODE_ALL_ERR_MSG: &str = "Input buffer has still data left after decoding!";

pub trait DecodeAll: Sized {
	
	fn decode_all(input: &mut &[u8]) -> Result<Self, Error>;
}

impl<T: Decode> DecodeAll for T {
	fn decode_all(input: &mut &[u8]) -> Result<Self, Error> { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Compact, Encode, EncodeLike, Input};

	macro_rules! test_decode_all {
		(
			$( $type:ty => $value:expr; )*
		) => {
			$(
				{
					let mut encoded = <$type as Encode>::encode(&$value);
					<$type>::decode_all(&mut encoded.as_slice()).expect(
						&format!("`{} => {}` decodes all!", stringify!($type), stringify!($value)),
					);

					encoded.extend(&[1, 2, 3, 4, 5, 6]);
					assert_eq!(
						<$type>::decode_all(&mut encoded.as_slice()).unwrap_err().to_string(),
						"Input buffer has still data left after decoding!",
					);
				}
			)*
		};
	}

	#[derive(Debug)]
	struct TestStruct {
		data: Vec<u32>,
		other: u8,
		compact: Compact<u128>,
	}

	impl EncodeLike for TestStruct {}

	impl Encode for TestStruct {
		fn encode(&self) -> Vec<u8> {
			let mut res = Vec::new();
			self.data.encode_to(&mut res);
			self.other.encode_to(&mut res);
			self.compact.encode_to(&mut res);
			res
		}
	}

	impl Decode for TestStruct {
		fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
			Ok(Self {
				data: Vec::<u32>::decode(input)?,
				other: u8::decode(input)?,
				compact: Compact::<u128>::decode(input)?,
			})
		}
	}

	#[test]
	fn decode_all_works() {
		test_decode_all! {
			u8 => 120;
			u16 => 30;
			u32 => 1;
			u64 => 2343545;
			u128 => 34358394245459854;
			Vec<u8> => vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
			Vec<u32> => vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
			Compact<u32> => Compact(32445);
			Compact<u128> => Compact(34353454453545);
			TestStruct => TestStruct { data: vec![1, 2, 4, 5, 6], other: 45, compact: Compact(123234545) };
		}
	}
}
