
use core::iter::Extend;

use crate::codec::Codec;

pub trait Joiner {
	
	fn and<V: Codec + Sized>(self, value: &V) -> Self;
}

impl<T> Joiner for T
where
	T: for<'a> Extend<&'a u8>,
{
	fn and<V: Codec + Sized>(mut self, value: &V) -> Self { panic!("STUB: not implemented") }
}
