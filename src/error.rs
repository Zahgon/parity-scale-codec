
use crate::alloc::borrow::Cow;
#[cfg(feature = "chain-error")]
use crate::alloc::boxed::Box;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Error {
	#[cfg(feature = "chain-error")]
	cause: Option<Box<Error>>,
	#[cfg(feature = "chain-error")]
	desc: Cow<'static, str>,
}

impl Error {
	
	pub fn chain(self, desc: impl Into<Cow<'static, str>>) -> Self { panic!("STUB: not implemented") }

	#[cfg(feature = "chain-error")]
	fn display_with_indent(&self, indent: u32, f: &mut core::fmt::Formatter) -> core::fmt::Result { panic!("STUB: not implemented") }
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result { panic!("STUB: not implemented") }
}

impl From<&'static str> for Error {
	fn from(desc: &'static str) -> Error { panic!("STUB: not implemented") }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod tests {
	use crate::Error;

	#[test]
	fn test_full_error() {
		let msg: &str = "final type:\n\twrap cause:\n\t\troot cause\n";

		let error = Error::from("root cause").chain("wrap cause").chain("final type");

		assert_eq!(&error.to_string(), msg);
	}

	#[test]
	fn impl_std_error() {
		use std::error::Error as _;

		let error = Error::from("root cause").chain("wrap cause").chain("final type");
		let s = error.source().unwrap();

		assert_eq!(&s.to_string(), "wrap cause:\n\troot cause\n");
	}
}
