use derive_more::{Display, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, From)]
#[display("{self:?}")]
pub enum Error {
	#[from(String, &String, &str)]
	Custom(String),

	// -- Externals
	#[from]
	OpenAi(async_openai::error::OpenAIError),

	#[from]
	Json(serde_json::Error),

	// reduce the size of RpcCall by boxing it
	RpcCall(Box<rpc_router::CallError>),
}

// region:    --- Froms

impl From<rpc_router::CallError> for Error {
	fn from(val: rpc_router::CallError) -> Self {
		Self::RpcCall(Box::new(val))
	}
}

// endregion: --- Froms

// region:    --- Custom

impl Error {
	pub fn custom_from_err(err: impl std::error::Error) -> Self {
		Self::Custom(err.to_string())
	}

	pub fn custom(val: impl Into<String>) -> Self {
		Self::Custom(val.into())
	}
}

// endregion: --- Custom

// region:    --- Error Boilerplate

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

	use super::*;

	#[test]
	fn test_error_from_openai() -> Result<()> {
		// -- Setup & Fixtures
		let openai_err = async_openai::error::OpenAIError::InvalidArgument(
			"test argument error".to_string(),
		);

		// -- Exec
		let err: Error = openai_err.into();

		// -- Check
		assert!(matches!(err, Error::OpenAi(_)));

		Ok(())
	}

	#[test]
	fn test_error_from_json() -> Result<()> {
		// -- Setup & Fixtures
		let json_err = serde_json::from_str::<serde_json::Value>("invalid json")
			.err()
			.ok_or("Should be invalid json error")?;

		// -- Exec
		let err: Error = json_err.into();

		// -- Check
		assert!(matches!(err, Error::Json(_)));

		Ok(())
	}

	#[test]
	fn test_error_from_rpc_call() -> Result<()> {
		// -- Setup & Fixtures
		let call_err = rpc_router::CallError {
			id: serde_json::Value::Null,
			method: "test_method".to_string(),
			error: rpc_router::Error::MethodUnknown,
		};

		// -- Exec
		let err: Error = call_err.into();

		// -- Check
		assert!(matches!(err, Error::RpcCall(_)));

		Ok(())
	}

	#[test]
	fn test_error_custom_from_err_simple() -> Result<()> {
		// -- Setup & Fixtures
		let io_err = std::io::Error::other("Some io error");

		// -- Exec
		let err = Error::custom_from_err(io_err);

		// -- Check
		assert!(matches!(err, Error::Custom(_)));
		if let Error::Custom(msg) = err {
			assert_eq!(msg, "Some io error");
		}

		Ok(())
	}

	#[test]
	#[ignore]
	fn test_error_custom_simple() -> Result<()> {
		// -- Setup & Fixtures
		let msg = "Some custom error message";

		// -- Exec
		let err = Error::custom(msg);

		// -- Check
		assert!(matches!(err, Error::Custom(_)));
		if let Error::Custom(val) = err {
			assert_eq!(val, msg);
		}

		Ok(())
	}
}

// endregion: --- Tests
