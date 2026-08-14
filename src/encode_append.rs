
use core::iter::ExactSizeIterator;

use crate::{
	alloc::vec::Vec,
	compact::{Compact, CompactLen},
	encode_like::EncodeLike,
	Decode, Encode, Error,
};

pub trait EncodeAppend {
	
	type Item: Encode;

	fn append_or_new<EncodeLikeItem, I>(self_encoded: Vec<u8>, iter: I) -> Result<Vec<u8>, Error>
	where
		I: IntoIterator<Item = EncodeLikeItem>,
		EncodeLikeItem: EncodeLike<Self::Item>,
		I::IntoIter: ExactSizeIterator;
}

impl<T: Encode> EncodeAppend for Vec<T> {
	type Item = T;

	fn append_or_new<EncodeLikeItem, I>(self_encoded: Vec<u8>, iter: I) -> Result<Vec<u8>, Error>
	where
		I: IntoIterator<Item = EncodeLikeItem>,
		EncodeLikeItem: EncodeLike<Self::Item>,
		I::IntoIter: ExactSizeIterator,
	{ panic!("STUB: not implemented") }
}

impl<T: Encode> EncodeAppend for crate::alloc::collections::VecDeque<T> {
	type Item = T;

	fn append_or_new<EncodeLikeItem, I>(self_encoded: Vec<u8>, iter: I) -> Result<Vec<u8>, Error>
	where
		I: IntoIterator<Item = EncodeLikeItem>,
		EncodeLikeItem: EncodeLike<Self::Item>,
		I::IntoIter: ExactSizeIterator,
	{ panic!("STUB: not implemented") }
}

fn append_or_new_impl<Item, I>(mut vec: Vec<u8>, iter: I) -> Result<Vec<u8>, Error>
where
	Item: Encode,
	I: IntoIterator<Item = Item>,
	I::IntoIter: ExactSizeIterator,
{ panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Encode, EncodeLike, Input};
	use std::collections::VecDeque;

	const TEST_VALUE: u32 = {
		#[cfg(not(miri))]
		{
			1_000_000
		}
		#[cfg(miri)]
		{
			1_000
		}
	};

	#[test]
	fn vec_encode_append_works() {
		let encoded = (0..TEST_VALUE).fold(Vec::new(), |encoded, v| {
			<Vec<u32> as EncodeAppend>::append_or_new(encoded, std::iter::once(&v)).unwrap()
		});

		let decoded = Vec::<u32>::decode(&mut &encoded[..]).unwrap();
		assert_eq!(decoded, (0..TEST_VALUE).collect::<Vec<_>>());
	}

	#[test]
	fn vec_encode_append_multiple_items_works() {
		let encoded = (0..TEST_VALUE).fold(Vec::new(), |encoded, v| {
			<Vec<u32> as EncodeAppend>::append_or_new(encoded, [v, v, v, v]).unwrap()
		});

		let decoded = Vec::<u32>::decode(&mut &encoded[..]).unwrap();
		let expected = (0..TEST_VALUE).fold(Vec::new(), |mut vec, i| {
			vec.append(&mut vec![i, i, i, i]);
			vec
		});
		assert_eq!(decoded, expected);
	}

	#[test]
	fn vecdeque_encode_append_works() {
		let encoded = (0..TEST_VALUE).fold(Vec::new(), |encoded, v| {
			<VecDeque<u32> as EncodeAppend>::append_or_new(encoded, std::iter::once(&v)).unwrap()
		});

		let decoded = VecDeque::<u32>::decode(&mut &encoded[..]).unwrap();
		assert_eq!(decoded, (0..TEST_VALUE).collect::<Vec<_>>());
	}

	#[test]
	fn vecdeque_encode_append_multiple_items_works() {
		let encoded = (0..TEST_VALUE).fold(Vec::new(), |encoded, v| {
			<VecDeque<u32> as EncodeAppend>::append_or_new(encoded, [v, v, v, v]).unwrap()
		});

		let decoded = VecDeque::<u32>::decode(&mut &encoded[..]).unwrap();
		let expected = (0..TEST_VALUE).fold(Vec::new(), |mut vec, i| {
			vec.append(&mut vec![i, i, i, i]);
			vec
		});
		assert_eq!(decoded, expected);
	}

	#[test]
	fn append_non_copyable() {
		#[derive(Eq, PartialEq, Debug)]
		struct NoCopy {
			data: u32,
		}

		impl EncodeLike for NoCopy {}

		impl Encode for NoCopy {
			fn encode(&self) -> Vec<u8> {
				self.data.encode()
			}
		}

		impl Decode for NoCopy {
			fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
				u32::decode(input).map(|data| Self { data })
			}
		}

		let append = NoCopy { data: 100 };
		let data = Vec::new();
		let encoded =
			<Vec<NoCopy> as EncodeAppend>::append_or_new(data, std::iter::once(&append)).unwrap();

		let decoded = <Vec<NoCopy>>::decode(&mut &encoded[..]).unwrap();
		assert_eq!(vec![append], decoded);
	}

	#[test]
	fn vec_encode_like_append_works() {
		let encoded = (0..TEST_VALUE).fold(Vec::new(), |encoded, v| {
			<Vec<u32> as EncodeAppend>::append_or_new(encoded, std::iter::once(Box::new(v)))
				.unwrap()
		});

		let decoded = Vec::<u32>::decode(&mut &encoded[..]).unwrap();
		assert_eq!(decoded, (0..TEST_VALUE).collect::<Vec<_>>());
	}
}
