use async_openai::types::chat::ChatCompletionTools;
use rpc_router::Router;
use std::sync::Arc;

#[derive(Clone)]
pub struct AiTools {
	router: Router,
	chat_tools: Arc<Vec<ChatCompletionTools>>,
}

impl AiTools {
	pub fn new(router: Router, chat_tools: Vec<ChatCompletionTools>) -> Self {
		AiTools {
			router,
			chat_tools: Arc::new(chat_tools),
		}
	}
}

impl AiTools {
	pub fn router(&self) -> &Router {
		&self.router
	}

	pub fn chat_tools_clone(&self) -> Vec<ChatCompletionTools> {
		self.chat_tools.as_ref().clone()
	}
}
