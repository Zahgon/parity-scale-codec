
use core::marker::PhantomData;

pub struct DecodeFinished(PhantomData<*const ()>);

impl DecodeFinished {
	
	#[inline]
	pub unsafe fn assert_decoding_finished() -> DecodeFinished { panic!("STUB: not implemented") }
}
