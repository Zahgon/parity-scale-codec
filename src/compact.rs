
use arrayvec::ArrayVec;

use crate::{
	alloc::vec::Vec,
	codec::{Decode, Encode, EncodeAsRef, Input, Output},
	encode_like::EncodeLike,
	DecodeWithMemTracking, Error,
};

#[cfg(feature = "fuzz")]
use arbitrary::Arbitrary;

struct ArrayVecWrapper<const N: usize>(ArrayVec<u8, N>);

impl<const N: usize> Output for ArrayVecWrapper<N> {
	fn write(&mut self, bytes: &[u8]) { panic!("STUB: not implemented") }

	fn push_byte(&mut self, byte: u8) { panic!("STUB: not implemented") }
}

struct PrefixInput<'a, T> {
	prefix: Option<u8>,
	input: &'a mut T,
}

impl<'a, T: 'a + Input> Input for PrefixInput<'a, T> {
	fn remaining_len(&mut self) -> Result<Option<usize>, Error> { panic!("STUB: not implemented") }

	fn read(&mut self, buffer: &mut [u8]) -> Result<(), Error> { panic!("STUB: not implemented") }
}

pub trait CompactLen<T> {
	
	fn compact_len(val: &T) -> usize;
}

#[derive(Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
#[cfg_attr(feature = "fuzz", derive(Arbitrary))]
pub struct Compact<T>(pub T);

impl<T> From<T> for Compact<T> {
	fn from(x: T) -> Compact<T> { panic!("STUB: not implemented") }
}

impl<'a, T: Copy> From<&'a T> for Compact<T> {
	fn from(x: &'a T) -> Compact<T> { panic!("STUB: not implemented") }
}

pub trait CompactAs: From<Compact<Self>> {
	
	type As;

	fn encode_as(&self) -> &Self::As;

	fn decode_from(_: Self::As) -> Result<Self, Error>;
}

impl<T> EncodeLike for Compact<T> where for<'a> CompactRef<'a, T>: Encode {}

impl<T> Encode for Compact<T>
where
	for<'a> CompactRef<'a, T>: Encode,
{
	fn size_hint(&self) -> usize { panic!("STUB: not implemented") }

	fn encode_to<W: Output + ?Sized>(&self, dest: &mut W) { panic!("STUB: not implemented") }

	fn encode(&self) -> Vec<u8> { panic!("STUB: not implemented") }

	fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R { panic!("STUB: not implemented") }
}

impl<T> EncodeLike for CompactRef<'_, T>
where
	T: CompactAs,
	for<'b> CompactRef<'b, T::As>: Encode,
{
}

impl<T> Encode for CompactRef<'_, T>
where
	T: CompactAs,
	for<'b> CompactRef<'b, T::As>: Encode,
{
	fn size_hint(&self) -> usize { panic!("STUB: not implemented") }

	fn encode_to<Out: Output + ?Sized>(&self, dest: &mut Out) { panic!("STUB: not implemented") }

	fn encode(&self) -> Vec<u8> { panic!("STUB: not implemented") }

	fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R { panic!("STUB: not implemented") }
}

impl<T> Decode for Compact<T>
where
	T: CompactAs,
	Compact<T::As>: Decode,
{
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> { panic!("STUB: not implemented") }
}

impl<T> DecodeWithMemTracking for Compact<T>
where
	T: CompactAs,
	Compact<T::As>: DecodeWithMemTracking,
{
}

macro_rules! impl_from_compact {
	( $( $ty:ty ),* ) => {
		$(
			impl From<Compact<$ty>> for $ty {
				fn from(x: Compact<$ty>) -> $ty { x.0 }
			}
		)*
	}
}

impl_from_compact! { (), u8, u16, u32, u64, u128 }

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct CompactRef<'a, T>(pub &'a T);

impl<'a, T> From<&'a T> for CompactRef<'a, T> {
	fn from(x: &'a T) -> Self { panic!("STUB: not implemented") }
}

impl<T> core::fmt::Debug for Compact<T>
where
	T: core::fmt::Debug,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { panic!("STUB: not implemented") }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for Compact<T>
where
	T: serde::Serialize,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{ panic!("STUB: not implemented") }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for Compact<T>
where
	T: serde::Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{ panic!("STUB: not implemented") }
}

pub trait HasCompact: Sized {
	
	type Type: for<'a> EncodeAsRef<'a, Self> + Decode + From<Self> + Into<Self>;
}

impl<'a, T: 'a> EncodeAsRef<'a, T> for Compact<T>
where
	CompactRef<'a, T>: Encode + From<&'a T>,
{
	type RefType = CompactRef<'a, T>;
}

impl<T: 'static> HasCompact for T
where
	Compact<T>: for<'a> EncodeAsRef<'a, T> + Decode + From<Self> + Into<Self>,
{
	type Type = Compact<T>;
}

impl Encode for CompactRef<'_, ()> {
	fn encode_to<W: Output + ?Sized>(&self, _dest: &mut W) { panic!("STUB: not implemented") }

	fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R { panic!("STUB: not implemented") }

	fn encode(&self) -> Vec<u8> { panic!("STUB: not implemented") }
}

impl Encode for CompactRef<'_, u8> {
	fn size_hint(&self) -> usize { panic!("STUB: not implemented") }

	fn encode_to<W: Output + ?Sized>(&self, dest: &mut W) { panic!("STUB: not implemented") }

	fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R { panic!("STUB: not implemented") }
}

impl CompactLen<u8> for Compact<u8> {
	fn compact_len(val: &u8) -> usize { panic!("STUB: not implemented") }
}

impl Encode for CompactRef<'_, u16> {
	fn size_hint(&self) -> usize { panic!("STUB: not implemented") }

	fn encode_to<W: Output + ?Sized>(&self, dest: &mut W) { panic!("STUB: not implemented") }

	fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R { panic!("STUB: not implemented") }
}

impl CompactLen<u16> for Compact<u16> {
	fn compact_len(val: &u16) -> usize { panic!("STUB: not implemented") }
}

impl Encode for CompactRef<'_, u32> {
	fn size_hint(&self) -> usize { panic!("STUB: not implemented") }

	fn encode_to<W: Output + ?Sized>(&self, dest: &mut W) { panic!("STUB: not implemented") }

	fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R { panic!("STUB: not implemented") }
}

impl CompactLen<u32> for Compact<u32> {
	fn compact_len(val: &u32) -> usize { panic!("STUB: not implemented") }
}

impl Encode for CompactRef<'_, u64> {
	fn size_hint(&self) -> usize { panic!("STUB: not implemented") }

	fn encode_to<W: Output + ?Sized>(&self, dest: &mut W) { panic!("STUB: not implemented") }

	fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R { panic!("STUB: not implemented") }
}

impl CompactLen<u64> for Compact<u64> {
	fn compact_len(val: &u64) -> usize { panic!("STUB: not implemented") }
}

impl Encode for CompactRef<'_, u128> {
	fn size_hint(&self) -> usize { panic!("STUB: not implemented") }

	fn encode_to<W: Output + ?Sized>(&self, dest: &mut W) { panic!("STUB: not implemented") }

	fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R { panic!("STUB: not implemented") }
}

impl CompactLen<u128> for Compact<u128> {
	fn compact_len(val: &u128) -> usize { panic!("STUB: not implemented") }
}

impl Decode for Compact<()> {
	fn decode<I: Input>(_input: &mut I) -> Result<Self, Error> { panic!("STUB: not implemented") }
}

impl DecodeWithMemTracking for Compact<()> {}

const U8_OUT_OF_RANGE: &str = "out of range decoding Compact<u8>";
const U16_OUT_OF_RANGE: &str = "out of range decoding Compact<u16>";
const U32_OUT_OF_RANGE: &str = "out of range decoding Compact<u32>";
const U64_OUT_OF_RANGE: &str = "out of range decoding Compact<u64>";
const U128_OUT_OF_RANGE: &str = "out of range decoding Compact<u128>";

impl Decode for Compact<u8> {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> { panic!("STUB: not implemented") }
}

impl DecodeWithMemTracking for Compact<u8> {}

impl Decode for Compact<u16> {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> { panic!("STUB: not implemented") }
}

impl DecodeWithMemTracking for Compact<u16> {}

impl Decode for Compact<u32> {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> { panic!("STUB: not implemented") }
}

impl DecodeWithMemTracking for Compact<u32> {}

impl Decode for Compact<u64> {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> { panic!("STUB: not implemented") }
}

impl DecodeWithMemTracking for Compact<u64> {}

impl Decode for Compact<u128> {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> { panic!("STUB: not implemented") }
}

impl DecodeWithMemTracking for Compact<u128> {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn prefix_input_empty_read_unchanged() {
		let mut input = PrefixInput { prefix: Some(1), input: &mut &vec![2, 3, 4][..] };
		assert_eq!(input.remaining_len(), Ok(Some(4)));
		let mut empty_buf = [];
		assert_eq!(input.read(&mut empty_buf[..]), Ok(()));
		assert_eq!(input.remaining_len(), Ok(Some(4)));
		assert_eq!(input.read_byte(), Ok(1));
	}

	#[test]
	fn compact_128_encoding_works() {
		let tests = [
			(0u128, 1usize),
			(63, 1),
			(64, 2),
			(16383, 2),
			(16384, 4),
			(1073741823, 4),
			(1073741824, 5),
			((1 << 32) - 1, 5),
			(1 << 32, 6),
			(1 << 40, 7),
			(1 << 48, 8),
			((1 << 56) - 1, 8),
			(1 << 56, 9),
			((1 << 64) - 1, 9),
			(1 << 64, 10),
			(1 << 72, 11),
			(1 << 80, 12),
			(1 << 88, 13),
			(1 << 96, 14),
			(1 << 104, 15),
			(1 << 112, 16),
			((1 << 120) - 1, 16),
			(1 << 120, 17),
			(u128::MAX, 17),
		];
		for &(n, l) in &tests {
			let encoded = Compact(n).encode();
			assert_eq!(encoded.len(), l);
			assert_eq!(Compact::compact_len(&n), l);
			assert_eq!(<Compact<u128>>::decode(&mut &encoded[..]).unwrap().0, n);
		}
	}

	#[test]
	fn compact_64_encoding_works() {
		let tests = [
			(0u64, 1usize),
			(63, 1),
			(64, 2),
			(16383, 2),
			(16384, 4),
			(1073741823, 4),
			(1073741824, 5),
			((1 << 32) - 1, 5),
			(1 << 32, 6),
			(1 << 40, 7),
			(1 << 48, 8),
			((1 << 56) - 1, 8),
			(1 << 56, 9),
			(u64::MAX, 9),
		];
		for &(n, l) in &tests {
			let encoded = Compact(n).encode();
			assert_eq!(encoded.len(), l);
			assert_eq!(Compact::compact_len(&n), l);
			assert_eq!(<Compact<u64>>::decode(&mut &encoded[..]).unwrap().0, n);
		}
	}

	#[test]
	fn compact_32_encoding_works() {
		let tests = [
			(0u32, 1usize),
			(63, 1),
			(64, 2),
			(16383, 2),
			(16384, 4),
			(1073741823, 4),
			(1073741824, 5),
			(u32::MAX, 5),
		];
		for &(n, l) in &tests {
			let encoded = Compact(n).encode();
			assert_eq!(encoded.len(), l);
			assert_eq!(Compact::compact_len(&n), l);
			assert_eq!(<Compact<u32>>::decode(&mut &encoded[..]).unwrap().0, n);
		}
	}

	#[test]
	fn compact_16_encoding_works() {
		let tests = [(0u16, 1usize), (63, 1), (64, 2), (16383, 2), (16384, 4), (65535, 4)];
		for &(n, l) in &tests {
			let encoded = Compact(n).encode();
			assert_eq!(encoded.len(), l);
			assert_eq!(Compact::compact_len(&n), l);
			assert_eq!(<Compact<u16>>::decode(&mut &encoded[..]).unwrap().0, n);
		}
		assert!(<Compact<u16>>::decode(&mut &Compact(65536u32).encode()[..]).is_err());
	}

	#[test]
	fn compact_8_encoding_works() {
		let tests = [(0u8, 1usize), (63, 1), (64, 2), (255, 2)];
		for &(n, l) in &tests {
			let encoded = Compact(n).encode();
			assert_eq!(encoded.len(), l);
			assert_eq!(Compact::compact_len(&n), l);
			assert_eq!(<Compact<u8>>::decode(&mut &encoded[..]).unwrap().0, n);
		}
		assert!(<Compact<u8>>::decode(&mut &Compact(256u32).encode()[..]).is_err());
	}

	fn hexify(bytes: &[u8]) -> String {
		bytes
			.iter()
			.map(|ref b| format!("{:02x}", b))
			.collect::<Vec<String>>()
			.join(" ")
	}

	#[test]
	fn compact_integers_encoded_as_expected() {
		let tests = [
			(0u64, "00"),
			(63, "fc"),
			(64, "01 01"),
			(16383, "fd ff"),
			(16384, "02 00 01 00"),
			(1073741823, "fe ff ff ff"),
			(1073741824, "03 00 00 00 40"),
			((1 << 32) - 1, "03 ff ff ff ff"),
			(1 << 32, "07 00 00 00 00 01"),
			(1 << 40, "0b 00 00 00 00 00 01"),
			(1 << 48, "0f 00 00 00 00 00 00 01"),
			((1 << 56) - 1, "0f ff ff ff ff ff ff ff"),
			(1 << 56, "13 00 00 00 00 00 00 00 01"),
			(u64::MAX, "13 ff ff ff ff ff ff ff ff"),
		];
		for &(n, s) in &tests {
			
			let encoded = Compact(n).encode();
			assert_eq!(hexify(&encoded), s);
			assert_eq!(<Compact<u64>>::decode(&mut &encoded[..]).unwrap().0, n);

			if n <= u32::MAX as u64 {
				assert_eq!(<Compact<u32>>::decode(&mut &encoded[..]).unwrap().0, n as u32);
				let encoded = Compact(n as u32).encode();
				assert_eq!(hexify(&encoded), s);
				assert_eq!(<Compact<u64>>::decode(&mut &encoded[..]).unwrap().0, n);
			}
			if n <= u16::MAX as u64 {
				assert_eq!(<Compact<u16>>::decode(&mut &encoded[..]).unwrap().0, n as u16);
				let encoded = Compact(n as u16).encode();
				assert_eq!(hexify(&encoded), s);
				assert_eq!(<Compact<u64>>::decode(&mut &encoded[..]).unwrap().0, n);
			}
			if n <= u8::MAX as u64 {
				assert_eq!(<Compact<u8>>::decode(&mut &encoded[..]).unwrap().0, n as u8);
				let encoded = Compact(n as u8).encode();
				assert_eq!(hexify(&encoded), s);
				assert_eq!(<Compact<u64>>::decode(&mut &encoded[..]).unwrap().0, n);
			}
		}
	}

	#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
	#[derive(PartialEq, Eq, Clone)]
	struct Wrapper(u8);

	impl CompactAs for Wrapper {
		type As = u8;
		fn encode_as(&self) -> &u8 {
			&self.0
		}
		fn decode_from(x: u8) -> Result<Wrapper, Error> {
			Ok(Wrapper(x))
		}
	}

	impl From<Compact<Wrapper>> for Wrapper {
		fn from(x: Compact<Wrapper>) -> Wrapper {
			x.0
		}
	}

	#[test]
	fn compact_as_8_encoding_works() {
		let tests = [(0u8, 1usize), (63, 1), (64, 2), (255, 2)];
		for &(n, l) in &tests {
			let compact: Compact<Wrapper> = Wrapper(n).into();
			let encoded = compact.encode();
			assert_eq!(encoded.len(), l);
			assert_eq!(Compact::compact_len(&n), l);
			let decoded = <Compact<Wrapper>>::decode(&mut &encoded[..]).unwrap();
			let wrapper: Wrapper = decoded.into();
			assert_eq!(wrapper, Wrapper(n));
		}
	}

	struct WithCompact<T: HasCompact> {
		_data: T,
	}

	#[test]
	fn compact_as_has_compact() {
		let _data = WithCompact { _data: Wrapper(1) };
	}

	#[test]
	fn compact_using_encoded_arrayvec_size() {
		Compact(u8::MAX).using_encoded(|_| {});
		Compact(u16::MAX).using_encoded(|_| {});
		Compact(u32::MAX).using_encoded(|_| {});
		Compact(u64::MAX).using_encoded(|_| {});
		Compact(u128::MAX).using_encoded(|_| {});

		CompactRef(&u8::MAX).using_encoded(|_| {});
		CompactRef(&u16::MAX).using_encoded(|_| {});
		CompactRef(&u32::MAX).using_encoded(|_| {});
		CompactRef(&u64::MAX).using_encoded(|_| {});
		CompactRef(&u128::MAX).using_encoded(|_| {});
	}

	#[test]
	#[should_panic]
	fn array_vec_output_oob() {
		let mut v = ArrayVecWrapper(ArrayVec::<u8, 4>::new());
		v.write(&[1, 2, 3, 4, 5]);
	}

	#[test]
	fn array_vec_output() {
		let mut v = ArrayVecWrapper(ArrayVec::<u8, 4>::new());
		v.write(&[1, 2, 3, 4]);
	}

	macro_rules! check_bound {
		( $m:expr, $ty:ty, $typ1:ty, [ $(($ty2:ty, $ty2_err:expr)),* ]) => {
			$(
				check_bound!($m, $ty, $typ1, $ty2, $ty2_err);
			)*
		};
		( $m:expr, $ty:ty, $typ1:ty, $ty2:ty, $ty2_err:expr) => {
			let enc = ((<$ty>::MAX >> 2) as $typ1 << 2) | $m;
			assert_eq!(Compact::<$ty2>::decode(&mut &enc.to_le_bytes()[..]),
				Err($ty2_err.into()));
		};
	}
	macro_rules! check_bound_u32 {
		( [ $(($ty2:ty, $ty2_err:expr)),* ]) => {
			$(
				check_bound_u32!($ty2, $ty2_err);
			)*
		};
		( $ty2:ty, $ty2_err:expr ) => {
			assert_eq!(Compact::<$ty2>::decode(&mut &[0b11, 0xff, 0xff, 0xff, 0xff >> 2][..]),
				Err($ty2_err.into()));
		};
	}
	macro_rules! check_bound_high {
		( $m:expr, [ $(($ty2:ty, $ty2_err:expr)),* ]) => {
			$(
				check_bound_high!($m, $ty2, $ty2_err);
			)*
		};
		( $s:expr, $ty2:ty, $ty2_err:expr) => {
			let mut dest = Vec::new();
			dest.push(0b11 + (($s - 4) << 2) as u8);
			for _ in 0..($s - 1) {
				dest.push(u8::MAX);
			}
			dest.push(0);
			assert_eq!(Compact::<$ty2>::decode(&mut &dest[..]),
				Err($ty2_err.into()));
		};
	}

	#[test]
	fn compact_u64_test() {
		for a in [
			u64::MAX,
			u64::MAX - 1,
			u64::MAX << 8,
			(u64::MAX << 8) - 1,
			u64::MAX << 16,
			(u64::MAX << 16) - 1,
		]
		.iter()
		{
			let e = Compact::<u64>::encode(&Compact(*a));
			let d = Compact::<u64>::decode(&mut &e[..]).unwrap().0;
			assert_eq!(*a, d);
		}
	}

	#[test]
	fn compact_u128_test() {
		for a in [u64::MAX as u128, (u64::MAX - 10) as u128, u128::MAX, u128::MAX - 10].iter() {
			let e = Compact::<u128>::encode(&Compact(*a));
			let d = Compact::<u128>::decode(&mut &e[..]).unwrap().0;
			assert_eq!(*a, d);
		}
	}

	#[test]
	fn should_avoid_overlapping_definition() {
		check_bound!(
			0b01,
			u8,
			u16,
			[
				(u8, U8_OUT_OF_RANGE),
				(u16, U16_OUT_OF_RANGE),
				(u32, U32_OUT_OF_RANGE),
				(u64, U64_OUT_OF_RANGE),
				(u128, U128_OUT_OF_RANGE)
			]
		);
		check_bound!(
			0b10,
			u16,
			u32,
			[
				(u16, U16_OUT_OF_RANGE),
				(u32, U32_OUT_OF_RANGE),
				(u64, U64_OUT_OF_RANGE),
				(u128, U128_OUT_OF_RANGE)
			]
		);
		check_bound_u32!([
			(u32, U32_OUT_OF_RANGE),
			(u64, U64_OUT_OF_RANGE),
			(u128, U128_OUT_OF_RANGE)
		]);
		for i in 5..=8 {
			check_bound_high!(i, [(u64, U64_OUT_OF_RANGE), (u128, U128_OUT_OF_RANGE)]);
		}
		for i in 8..=16 {
			check_bound_high!(i, [(u128, U128_OUT_OF_RANGE)]);
		}
	}

	macro_rules! quick_check_roundtrip {
		( $( $ty:ty : $test:ident ),* ) => {
			$(
				quickcheck::quickcheck! {
					fn $test(v: $ty) -> bool {
						let encoded = Compact(v).encode();
						let deencoded = <Compact<$ty>>::decode(&mut &encoded[..]).unwrap().0;

						v == deencoded
					}
				}
			)*
		}
	}

	quick_check_roundtrip! {
		u8: u8_roundtrip,
		u16: u16_roundtrip,
		u32 : u32_roundtrip,
		u64 : u64_roundtrip,
		u128 : u128_roundtrip
	}
}
