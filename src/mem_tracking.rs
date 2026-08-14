
use crate::{Decode, Error, Input};
use impl_trait_for_tuples::impl_for_tuples;

pub trait DecodeWithMemTracking: Decode {}

const DECODE_OOM_MSG: &str = "Heap memory limit exceeded while decoding";

#[impl_for_tuples(18)]
impl DecodeWithMemTracking for Tuple {}

pub struct MemTrackingInput<'a, I> {
	input: &'a mut I,
	used_mem: usize,
	mem_limit: usize,
}

impl<'a, I: Input> MemTrackingInput<'a, I> {
	
	pub fn new(input: &'a mut I, mem_limit: usize) -> Self { panic!("STUB: not implemented") }

	pub fn used_mem(&self) -> usize { panic!("STUB: not implemented") }
}

impl<I: Input> Input for MemTrackingInput<'_, I> {
	fn remaining_len(&mut self) -> Result<Option<usize>, Error> { panic!("STUB: not implemented") }

	fn read(&mut self, into: &mut [u8]) -> Result<(), Error> { panic!("STUB: not implemented") }

	fn read_byte(&mut self) -> Result<u8, Error> { panic!("STUB: not implemented") }

	fn descend_ref(&mut self) -> Result<(), Error> { panic!("STUB: not implemented") }

	fn ascend_ref(&mut self) { panic!("STUB: not implemented") }

	fn on_before_alloc_mem(&mut self, size: usize) -> Result<(), Error> { panic!("STUB: not implemented") }
}

pub trait DecodeWithMemLimit: DecodeWithMemTracking {
	
	fn decode_with_mem_limit<I: Input>(input: &mut I, mem_limit: usize) -> Result<Self, Error>;
}

impl<T> DecodeWithMemLimit for T
where
	T: DecodeWithMemTracking,
{
	fn decode_with_mem_limit<I: Input>(input: &mut I, mem_limit: usize) -> Result<Self, Error> { panic!("STUB: not implemented") }
}
