//! Mock opencode AI models (replica mode).
//!
//! - `POST /_mcp/tools/call` answers the 47 tools of the opencode models
//!   MCP server (`gateway/opencode_mcp.py`, snapshot
//!   `servers/snapshots/opencode.json`): `opencode_list_models`,
//!   `opencode_chat`, `opencode_default_model`, `opencode_get_model`
//!   plus one bound chat tool per live opencode model (e.g.
//!   `chat_opencode_go_beam_free`).
//! - Responses are deterministic canned echoes (no live LLM, no
//!   network): chat tools reply `[mock opencode] <ref> echoes: <prompt>`.
//! - Catalog wiring (`catalog/services.toml`): service `opencode`,
//!   `mode = "replica"`, `world_port = 8131`, endpoint `opencode`
//!   at `mcp_port = 7133`.

pub mod tools;

use axum::{Json, Router, extract::State, routing::post};
use mock_core::{
    mcp::{self, TOOLS_CALL_PATH, ToolCall},
    service::MockService,
    world::SharedWorld,
};
use serde_json::{Value, json as json_value};

pub const NAME: &str = "opencode";
pub const PORT: u16 = 8131;

pub struct Opencode;

impl MockService for Opencode {
    fn name(&self) -> &'static str {
        NAME
    }

    fn port(&self) -> u16 {
        PORT
    }

    fn hosts(&self) -> &'static [&'static str] {
        &[]
    }

    fn router(&self) -> Router<SharedWorld> {
        Router::new().route(TOOLS_CALL_PATH, post(tools_call))
    }
}

/// Replica tool calls (see `servers/snapshots/opencode.json` for the tool list).
async fn tools_call(State(_world): State<SharedWorld>, Json(call): Json<ToolCall>) -> Json<Value> {
    let args = match call.arguments {
        Value::Null => json_value!({}),
        other => other,
    };
    let result = tools::dispatch(&call.name, &args);
    Json(match result {
        Some(Ok(v)) => mcp::text(v.to_string()),
        Some(Err(e)) => mcp::error(e),
        None => mcp::error(format!("Unknown tool: {}", call.name)),
    })
}
