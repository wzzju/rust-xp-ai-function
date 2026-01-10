use crate::Result;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use std::sync::Arc;

pub type OaClient = Arc<Client<OpenAIConfig>>;

pub fn new_oa_client() -> Result<OaClient> {
	dotenv::dotenv().ok();
	// OpenAIConfig automatically reads OPENAI_API_KEY
	let mut config = OpenAIConfig::new();

	// if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
	// 	config = config.with_api_key(api_key);
	// }

	if let Ok(api_base) = std::env::var("OPENAI_API_BASE") {
		config = config.with_api_base(api_base);
	}

	Ok(Client::with_config(config).into())
}
