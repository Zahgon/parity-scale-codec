
use crate::{alloc::boxed::Box, MaxEncodedLen};
use core::{
	marker::PhantomData,
	num::*,
	ops::{Range, RangeInclusive},
	time::Duration,
};
use impl_trait_for_tuples::impl_for_tuples;

pub trait ConstEncodedLen: MaxEncodedLen {}

#[impl_for_tuples(18)]
impl ConstEncodedLen for Tuple {}

impl<T: ConstEncodedLen, const N: usize> ConstEncodedLen for [T; N] {}

macro_rules! mark_cel {
	( $($n:ident <$t:ident>),+ ) => {
		$(
			impl<$t: ConstEncodedLen> ConstEncodedLen for $n<$t> { }
		)+
	};
	( $($t:ty),+ ) => {
		$(
			impl ConstEncodedLen for $t { }
		)+
	};
}

mark_cel!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool);
mark_cel!(
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

mark_cel!(Duration);
mark_cel!(PhantomData<T>);
mark_cel!(Box<T>);
mark_cel!(Range<T>, RangeInclusive<T>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Encode;
	use proptest::prelude::*;

	macro_rules! test_cel_compliance {
		( $( $t:ty ),+ ) => {
			$(
				paste::paste! {
					proptest::proptest! {
						#[test]
						fn [< cel_compliance_ $t:snake >](x: $t) {
							prop_assert_eq!(x.encode().len(), $t::max_encoded_len());
						}
					}
				}
			)*
		};
	}

	type Void = ();
	test_cel_compliance!(Void);

	test_cel_compliance!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool);

	type TupleArithmetic = (u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
	test_cel_compliance!(TupleArithmetic);

	test_cel_compliance!(
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

	type TupleNonZero = (
		NonZeroU8,
		NonZeroU16,
		NonZeroU32,
		NonZeroU64,
		NonZeroU128,
		NonZeroI8,
		NonZeroI16,
		NonZeroI32,
		NonZeroI64,
		NonZeroI128,
	);
	test_cel_compliance!(TupleNonZero);

	type ArrayArithmetic = [(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128); 10];
	test_cel_compliance!(ArrayArithmetic);

	test_cel_compliance!(Duration);

	type BoxedArithmetic = Box<(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128)>;
	test_cel_compliance!(BoxedArithmetic);

	type PhantomArithmetic = PhantomData<(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128)>;
	test_cel_compliance!(PhantomArithmetic);

	type Ranges = (Range<u8>, Range<u16>, Range<u32>, Range<u64>, Range<u128>);
	test_cel_compliance!(Ranges);

	type Ranges2D = (
		Range<(u8, u8)>,
		Range<(u16, u16)>,
		Range<(u32, u32)>,
		Range<(u64, u64)>,
		Range<(u128, u128)>,
	);
	test_cel_compliance!(Ranges2D);

	type RangesInc = (
		RangeInclusive<u8>,
		RangeInclusive<u16>,
		RangeInclusive<u32>,
		RangeInclusive<u64>,
		RangeInclusive<u128>,
	);
	test_cel_compliance!(RangesInc);

	type RangesInc2D = (
		RangeInclusive<(u8, u8)>,
		RangeInclusive<(u16, u16)>,
		RangeInclusive<(u32, u32)>,
		RangeInclusive<(u64, u64)>,
		RangeInclusive<(u128, u128)>,
	);
	test_cel_compliance!(RangesInc2D);
}
