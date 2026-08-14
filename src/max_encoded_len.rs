
use crate::{alloc::boxed::Box, Compact, Encode};
use core::{
	marker::PhantomData,
	mem,
	num::*,
	ops::{Range, RangeInclusive},
	time::Duration,
};
use impl_trait_for_tuples::impl_for_tuples;

#[cfg(target_has_atomic = "ptr")]
use crate::alloc::sync::Arc;

pub trait MaxEncodedLen: Encode {
	
	fn max_encoded_len() -> usize;
}

macro_rules! impl_primitives {
	( $($t:ty),+ ) => {
		$(
			impl MaxEncodedLen for $t {
				fn max_encoded_len() -> usize {
					mem::size_of::<$t>()
				}
			}
		)+
	};
}

impl_primitives!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool);

impl_primitives!(
	NonZeroU8,
	NonZeroU16,
	NonZeroU32,
	NonZeroU64,
	NonZeroU128,
	NonZeroI8,
	NonZeroI16,
	NonZeroI32,
	NonZeroI64,
	NonZeroI128
);

macro_rules! impl_compact {
	($( $t:ty => $e:expr; )*) => {
		$(
			impl MaxEncodedLen for Compact<$t> {
				fn max_encoded_len() -> usize {
					$e
				}
			}
		)*
	};
}

impl_compact!(
	() => 0;
	
	u8 => 2;
	
	u16 => 4;
	
	u32 => 5;
	
	u64 => 9;
	
	u128 => 17;
);

#[impl_for_tuples(18)]
impl MaxEncodedLen for Tuple {
	fn max_encoded_len() -> usize { panic!("STUB: not implemented") }
}

impl<T: MaxEncodedLen, const N: usize> MaxEncodedLen for [T; N] {
	fn max_encoded_len() -> usize { panic!("STUB: not implemented") }
}

impl<T: MaxEncodedLen> MaxEncodedLen for Box<T> {
	fn max_encoded_len() -> usize { panic!("STUB: not implemented") }
}

#[cfg(target_has_atomic = "ptr")]
impl<T: MaxEncodedLen> MaxEncodedLen for Arc<T> {
	fn max_encoded_len() -> usize { panic!("STUB: not implemented") }
}

impl<T: MaxEncodedLen> MaxEncodedLen for Option<T> {
	fn max_encoded_len() -> usize { panic!("STUB: not implemented") }
}

impl<T, E> MaxEncodedLen for Result<T, E>
where
	T: MaxEncodedLen,
	E: MaxEncodedLen,
{
	fn max_encoded_len() -> usize { panic!("STUB: not implemented") }
}

impl<T> MaxEncodedLen for PhantomData<T> {
	fn max_encoded_len() -> usize { panic!("STUB: not implemented") }
}

impl MaxEncodedLen for Duration {
	fn max_encoded_len() -> usize { panic!("STUB: not implemented") }
}

impl<T: MaxEncodedLen> MaxEncodedLen for Range<T> {
	fn max_encoded_len() -> usize { panic!("STUB: not implemented") }
}

impl<T: MaxEncodedLen> MaxEncodedLen for RangeInclusive<T> {
	fn max_encoded_len() -> usize { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
	use super::*;

	macro_rules! test_compact_length {
		($(fn $name:ident($t:ty);)*) => {
			$(
				#[test]
				fn $name() {
					assert_eq!(Compact(<$t>::MAX).encode().len(), Compact::<$t>::max_encoded_len());
				}
			)*
		};
	}

	test_compact_length!(
		fn compact_u8(u8);
		fn compact_u16(u16);
		fn compact_u32(u32);
		fn compact_u64(u64);
		fn compact_u128(u128);
	);
}
