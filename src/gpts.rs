use std::sync::LazyLock;
// use std::sync::OnceLock;

// pub fn model() -> &'static str {
// 	static MODEL: OnceLock<String> = OnceLock::new();
// 	MODEL.get_or_init(|| {
// 		std::env::var("MODEL").unwrap_or_else(|_| MODEL_3_TURBO.to_string())
// 	})
// }

pub fn model() -> &'static str {
	// directly bind initialization logic during definition
	static MODEL: LazyLock<String> = LazyLock::new(|| {
		std::env::var("MODEL").unwrap_or_else(|_| MODEL_3_TURBO.to_string())
	});

	// LazyLock implements Deref trait, supporting automatic dereferencing
	&MODEL
}

// -- GPT 4 Turbo

// Should use this one if want to use 4
pub const MODEL_4_O: &str = "gpt-4o";

// Should be legacy model at this point
// Typically point to the latest (as of 2024-03-13 - "gpt-4-0125-preview")
pub const MODEL_4_TURBO: &str = "gpt-4-turbo";

// -- GPT 3.5 Turbo

// Typically point to the latest (as of 2024-03-13 - "gpt-3.5-turbo-0125")
pub const MODEL_3_TURBO: &str = "gpt-3.5-turbo";
