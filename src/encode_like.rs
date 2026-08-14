
use crate::codec::Encode;

pub trait EncodeLike<T: Encode = Self>: Sized + Encode {}

pub struct Ref<'a, T: EncodeLike<U>, U: Encode>(&'a T, core::marker::PhantomData<U>);
impl<T: EncodeLike<U>, U: Encode> core::ops::Deref for Ref<'_, T, U> {
	type Target = T;
	fn deref(&self) -> &Self::Target { panic!("STUB: not implemented") }
}

impl<'a, T: EncodeLike<U>, U: Encode> From<&'a T> for Ref<'a, T, U> {
	fn from(x: &'a T) -> Self { panic!("STUB: not implemented") }
}
impl<T: EncodeLike<U>, U: Encode> crate::WrapperTypeEncode for Ref<'_, T, U> {}
impl<T: EncodeLike<U>, U: Encode> EncodeLike<U> for Ref<'_, T, U> {}
impl<T: EncodeLike<U>, U: Encode> EncodeLike<U> for &Ref<'_, T, U> {}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::BTreeMap;

	struct ComplexStuff<T>(T);

	impl<T: Encode> ComplexStuff<T> {
		fn complex_method<R: Encode>(value: &R) -> Vec<u8>
		where
			T: EncodeLike<R>,
		{
			value.encode()
		}
	}

	#[test]
	fn vec_and_slice_are_working() {
		let slice: &[u8] = &[1, 2, 3, 4];
		let data: Vec<u8> = slice.to_vec();

		let data_encoded = data.encode();
		let slice_encoded = ComplexStuff::<Vec<u8>>::complex_method(&slice);

		assert_eq!(slice_encoded, data_encoded);
	}

	#[test]
	fn btreemap_and_slice_are_working() {
		let slice: &[(u32, u32)] = &[(1, 2), (23, 24), (28, 30), (45, 80)];
		let data: BTreeMap<u32, u32> = slice.iter().copied().collect();

		let data_encoded = data.encode();
		let slice_encoded = ComplexStuff::<BTreeMap<u32, u32>>::complex_method(&slice);

		assert_eq!(slice_encoded, data_encoded);
	}

	#[test]
	fn interface_testing() {
		let value = 10u32;
		let data = (value, value, value);
		let encoded = ComplexStuff::<(u32, u32, u32)>::complex_method(&data);
		assert_eq!(data.encode(), encoded);
		let data = (&value, &value, &value);
		let encoded = ComplexStuff::<(u32, u32, u32)>::complex_method(&data);
		assert_eq!(data.encode(), encoded);
		let data = (&value, value, &value);
		let encoded = ComplexStuff::<(u32, u32, u32)>::complex_method(&data);
		assert_eq!(data.encode(), encoded);

		let vec_data: Vec<u8> = vec![1, 2, 3];
		ComplexStuff::<Vec<u8>>::complex_method(&vec_data);
		ComplexStuff::<&'static str>::complex_method(&String::from("test"));
		ComplexStuff::<&'static str>::complex_method(&"test");

		let slice: &[u8] = &vec_data;
		assert_eq!(
			ComplexStuff::<(u32, Vec<u8>)>::complex_method(&(1u32, slice.to_vec())),
			ComplexStuff::<(u32, Vec<u8>)>::complex_method(&(1u32, slice))
		);
	}
}
