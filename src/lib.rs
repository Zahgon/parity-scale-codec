
#![cfg_attr(all(test, nightly), feature(allocator_api))]
#![cfg_attr(all(test, nightly), feature(btreemap_alloc))]

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]

pub extern crate alloc;

#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate parity_scale_codec_derive;

#[cfg(all(feature = "std", test))]
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "derive")]
pub use parity_scale_codec_derive::*;

#[cfg(feature = "std")]

pub mod alloc {
	pub use std::{alloc, borrow, boxed, collections, rc, string, sync, vec};
}

pub mod __private {
	pub use const_format::concatcp;
}

#[cfg(feature = "bit-vec")]
mod bit_vec;
mod btree_utils;
mod codec;
mod compact;
#[cfg(feature = "max-encoded-len")]
mod const_encoded_len;
mod counted_input;
mod decode_all;
mod decode_finished;
mod depth_limit;
mod encode_append;
mod encode_like;
mod error;
#[cfg(feature = "generic-array")]
mod generic_array;
mod joiner;
mod keyedvec;
#[cfg(feature = "max-encoded-len")]
mod max_encoded_len;
mod mem_tracking;

#[cfg(feature = "std")]
pub use self::codec::IoReader;
pub use self::{
	codec::{
		decode_vec_with_len, Codec, Decode, DecodeLength, Encode, EncodeAsRef, FullCodec,
		FullEncode, Input, OptionBool, Output, WrapperTypeDecode, WrapperTypeEncode,
	},
	compact::{Compact, CompactAs, CompactLen, CompactRef, HasCompact},
	counted_input::CountedInput,
	decode_all::DecodeAll,
	decode_finished::DecodeFinished,
	depth_limit::DecodeLimit,
	encode_append::EncodeAppend,
	encode_like::{EncodeLike, Ref},
	error::Error,
	joiner::Joiner,
	keyedvec::KeyedVec,
	mem_tracking::{DecodeWithMemLimit, DecodeWithMemTracking, MemTrackingInput},
};
#[cfg(feature = "max-encoded-len")]
pub use const_encoded_len::ConstEncodedLen;
#[cfg(feature = "max-encoded-len")]
pub use max_encoded_len::MaxEncodedLen;

#[cfg(all(feature = "derive", feature = "max-encoded-len"))]
pub use parity_scale_codec_derive::MaxEncodedLen;

#[cfg(feature = "bytes")]
pub use self::codec::decode_from_bytes;
