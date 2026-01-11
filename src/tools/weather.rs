use crate::chat;
use crate::model::ModelManager;
use async_openai::types::chat::ChatCompletionTools;
use rpc_router::{RouterBuilder, RpcParams, router_builder};
use serde::{Deserialize, Serialize};

// region:    --- Builders

pub(super) fn router_builder() -> RouterBuilder {
	router_builder![get_weather]
}

pub(super) fn chat_tools() -> crate::Result<Vec<ChatCompletionTools>> {
	let tool_weather = chat::tool_fn_from_type::<GetWeatherParams>()?;

	Ok(vec![tool_weather])
}

// endregion: --- Builders

// region:    --- Weather Params

/// # get_weather
/// get the weather for a city
#[allow(unused)] // Will be passthrough API
#[derive(Debug, Deserialize, RpcParams, schemars::JsonSchema)]
struct GetWeatherParams {
	/// The city and state, e.g. San Francisco, CA
	location: String,
	/// The full country name of the city
	country: String,
	/// The temperature unit (e.g., 'celsius' or 'fahrenheit')
	unit: TempUnit,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema, RpcParams)]
#[serde(rename_all = "lowercase")]
enum TempUnit {
	Celsius,
	Fahrenheit,
}

#[derive(Serialize)]
struct Weather {
	temperature: f64,
	unit: TempUnit,
	humidity_rh: f32,
}

// endregion: --- Weather Params

// region:    --- Weather Function

async fn get_weather(
	_mm: ModelManager,
	params: GetWeatherParams,
) -> Result<Weather, String> {
	Ok(Weather {
		temperature: 30.,
		unit: params.unit,
		humidity_rh: 0.3,
	})
}

// endregion: --- Weather Function
