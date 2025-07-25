use ::http::response;
use bytes::Bytes;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::http::{PolicyResponse, filters};
use crate::http::{Request, Response, StatusCode};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct OAuthProtectedResource {
	/// Key-value pairs to include in the OAuth 2.0 Resource Server metadata response
	pub metadata: IndexMap<String, serde_json::Value>,
}

impl OAuthProtectedResource {
	pub fn apply(&self, req: &Request) -> Result<PolicyResponse, filters::Error> {
		// Serialize the metadata to JSON
		let json_body = match serde_json::to_string_pretty(&self.metadata) {
			Ok(json) => json,
			Err(_) => return Ok(PolicyResponse::default()), // Fall through on serialization error
		};

		// Create the response with proper JSON content type and CORS headers
		let response = response::Builder::new()
			.status(StatusCode::OK)
			.header("content-type", "application/json")
			.header("cache-control", "public, max-age=3600")
			.header("access-control-allow-origin", "*")
			.header("access-control-allow-methods", "GET, OPTIONS")
			.header("access-control-allow-headers", "content-type")
			.body(http::Body::from(Bytes::from(json_body)))
			.map_err(filters::Error::Http)?;

		Ok(PolicyResponse {
			direct_response: Some(response),
			response_headers: None,
		})
	}
}
