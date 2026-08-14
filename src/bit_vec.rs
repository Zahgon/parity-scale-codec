
use crate::{
	codec::decode_vec_with_len, Compact, Decode, DecodeWithMemTracking, Encode, EncodeLike, Error,
	Input, Output,
};
use bitvec::{
	boxed::BitBox, order::BitOrder, slice::BitSlice, store::BitStore, vec::BitVec, view::BitView,
};

impl<O: BitOrder, T: BitStore + Encode> Encode for BitSlice<T, O> {
	fn encode_to<W: Output + ?Sized>(&self, dest: &mut W) { panic!("STUB: not implemented") }
}

impl<O: BitOrder, T: BitStore + Encode> Encode for BitVec<T, O> {
	fn encode_to<W: Output + ?Sized>(&self, dest: &mut W) { panic!("STUB: not implemented") }
}

impl<O: BitOrder, T: BitStore + Encode> EncodeLike for BitVec<T, O> {}

const ARCH32BIT_BITSLICE_MAX_BITS: usize = 0x1fff_ffff;

impl<O: BitOrder, T: BitStore + Decode> Decode for BitVec<T, O> {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> { panic!("STUB: not implemented") }
}

impl<O: BitOrder, T: BitStore + Decode> DecodeWithMemTracking for BitVec<T, O> {}

impl<O: BitOrder, T: BitStore + Encode> Encode for BitBox<T, O> {
	fn encode_to<W: Output + ?Sized>(&self, dest: &mut W) { panic!("STUB: not implemented") }
}

impl<O: BitOrder, T: BitStore + Encode> EncodeLike for BitBox<T, O> {}

impl<O: BitOrder, T: BitStore + Decode> Decode for BitBox<T, O> {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> { panic!("STUB: not implemented") }
}

impl<O: BitOrder, T: BitStore + Decode> DecodeWithMemTracking for BitBox<T, O> {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{codec::INITIAL_PREALLOCATION, CompactLen};
	use bitvec::{
		bitvec,
		order::{Lsb0, Msb0},
	};

	macro_rules! test_data {
		($inner_type:ident) => (
			[
				BitVec::<$inner_type, Msb0>::new(),
				bitvec![$inner_type, Msb0; 0],
				bitvec![$inner_type, Msb0; 1],
				bitvec![$inner_type, Msb0; 0, 0],
				bitvec![$inner_type, Msb0; 1, 0],
				bitvec![$inner_type, Msb0; 0, 1],
				bitvec![$inner_type, Msb0; 1, 1],
				bitvec![$inner_type, Msb0; 1, 0, 1],
				bitvec![$inner_type, Msb0; 0, 1, 0, 1, 0, 1, 1],
				bitvec![$inner_type, Msb0; 0, 1, 0, 1, 0, 1, 1, 0],
				bitvec![$inner_type, Msb0; 1, 1, 0, 1, 0, 1, 1, 0, 1],
				bitvec![$inner_type, Msb0; 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0],
				bitvec![$inner_type, Msb0; 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0],
				bitvec![$inner_type, Msb0; 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0],
				bitvec![$inner_type, Msb0; 0; 15],
				bitvec![$inner_type, Msb0; 1; 16],
				bitvec![$inner_type, Msb0; 0; 17],
				bitvec![$inner_type, Msb0; 1; 31],
				bitvec![$inner_type, Msb0; 0; 32],
				bitvec![$inner_type, Msb0; 1; 33],
				bitvec![$inner_type, Msb0; 0; 63],
				bitvec![$inner_type, Msb0; 1; 64],
				bitvec![$inner_type, Msb0; 0; 65],
				bitvec![$inner_type, Msb0; 1; INITIAL_PREALLOCATION * 8 + 1],
				bitvec![$inner_type, Msb0; 0; INITIAL_PREALLOCATION * 9],
				bitvec![$inner_type, Msb0; 1; INITIAL_PREALLOCATION * 32 + 1],
				bitvec![$inner_type, Msb0; 0; INITIAL_PREALLOCATION * 33],
			]
		)
	}

	#[test]
	fn bitvec_u8() {
		for v in &test_data!(u8) {
			let encoded = v.encode();
			assert_eq!(*v, BitVec::<u8, Msb0>::decode(&mut &encoded[..]).unwrap());

			let elements = bitvec::mem::elts::<u8>(v.len());
			let compact_len = Compact::compact_len(&(v.len() as u32));
			assert_eq!(compact_len + elements, encoded.len(), "{}", v);
		}
	}

	#[test]
	fn bitvec_u16() {
		for v in &test_data!(u16) {
			let encoded = v.encode();
			assert_eq!(*v, BitVec::<u16, Msb0>::decode(&mut &encoded[..]).unwrap());

			let elements = bitvec::mem::elts::<u16>(v.len());
			let compact_len = Compact::compact_len(&(v.len() as u32));
			assert_eq!(compact_len + elements * 2, encoded.len(), "{}", v);
		}
	}

	#[test]
	fn bitvec_u32() {
		for v in &test_data!(u32) {
			let encoded = v.encode();
			assert_eq!(*v, BitVec::<u32, Msb0>::decode(&mut &encoded[..]).unwrap());

			let elements = bitvec::mem::elts::<u32>(v.len());
			let compact_len = Compact::compact_len(&(v.len() as u32));
			assert_eq!(compact_len + elements * 4, encoded.len(), "{}", v);
		}
	}

	#[test]
	fn bitvec_u64() {
		for v in &test_data!(u64) {
			let encoded = v.encode();
			assert_eq!(*v, BitVec::<u64, Msb0>::decode(&mut &encoded[..]).unwrap());

			let elements = bitvec::mem::elts::<u64>(v.len());
			let compact_len = Compact::compact_len(&(v.len() as u32));
			assert_eq!(compact_len + elements * 8, encoded.len(), "{}", v);
		}
	}

	#[test]
	fn bitslice() {
		let data: &[u8] = &[0x69];
		let slice = BitSlice::<u8, Msb0>::from_slice(data);
		let encoded = slice.encode();
		let decoded = BitVec::<u8, Msb0>::decode(&mut &encoded[..]).unwrap();
		assert_eq!(slice, decoded.as_bitslice());
	}

	#[test]
	fn bitbox() {
		let data: &[u8] = &[5, 10];
		let slice = BitSlice::<u8, Msb0>::from_slice(data);
		let bb = BitBox::<u8, Msb0>::from_bitslice(slice);
		let encoded = bb.encode();
		let decoded = BitBox::<u8, Msb0>::decode(&mut &encoded[..]).unwrap();
		assert_eq!(bb, decoded);
	}

	#[test]
	fn bitvec_u8_encodes_as_expected() {
		let cases = vec![
			(bitvec![u8, Lsb0; 0, 0, 1, 1].encode(), (Compact(4u32), 0b00001100u8).encode()),
			(bitvec![u8, Lsb0; 0, 1, 1, 1].encode(), (Compact(4u32), 0b00001110u8).encode()),
			(bitvec![u8, Lsb0; 1, 1, 1, 1].encode(), (Compact(4u32), 0b00001111u8).encode()),
			(bitvec![u8, Lsb0; 1, 1, 1, 1, 1].encode(), (Compact(5u32), 0b00011111u8).encode()),
			(bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0].encode(), (Compact(6u32), 0b00011111u8).encode()),
			(
				bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1].encode(),
				(Compact(12u32), 0b00011111u8, 0b00001011u8).encode(),
			),
		];

		for (idx, (actual, expected)) in cases.into_iter().enumerate() {
			assert_eq!(actual, expected, "case at index {} failed; encodings differ", idx);
		}
	}
}
