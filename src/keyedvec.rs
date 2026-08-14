
use core::iter::Extend;

use crate::{alloc::vec::Vec, codec::Codec};

pub trait KeyedVec {
	
	fn to_keyed_vec(&self, prepend_key: &[u8]) -> Vec<u8>;
}

impl<T: Codec> KeyedVec for T {
	fn to_keyed_vec(&self, prepend_key: &[u8]) -> Vec<u8> { panic!("STUB: not implemented") }
}
