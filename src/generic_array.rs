
use crate::{alloc::vec::Vec, encode_like::EncodeLike, Decode, Encode, Error, Input, Output};

impl<T: Encode, L: generic_array::ArrayLength> Encode for generic_array::GenericArray<T, L> {
	fn encode_to<W: Output + ?Sized>(&self, dest: &mut W) { panic!("STUB: not implemented") }
}

impl<T: Encode, L: generic_array::ArrayLength> EncodeLike for generic_array::GenericArray<T, L> {}

impl<T: Decode, L: generic_array::ArrayLength> Decode for generic_array::GenericArray<T, L> {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
	use super::*;
	use generic_array::{arr, typenum, GenericArray};

	#[test]
	fn generic_array() {
		let test = arr![3u8, 4, 5];
		let encoded = test.encode();
		assert_eq!(test, GenericArray::<u8, typenum::U3>::decode(&mut &encoded[..]).unwrap());

		let test = arr![3u16, 4, 5, 6, 7, 8, 0];
		let encoded = test.encode();
		assert_eq!(test, GenericArray::<u16, typenum::U7>::decode(&mut &encoded[..]).unwrap());

		let test = arr![3u32, 4, 5, 0, 1];
		let encoded = test.encode();
		assert_eq!(test, GenericArray::<u32, typenum::U5>::decode(&mut &encoded[..]).unwrap());

		let test = arr![3u64];
		let encoded = test.encode();
		assert_eq!(test, GenericArray::<u64, typenum::U1>::decode(&mut &encoded[..]).unwrap());
	}
}
