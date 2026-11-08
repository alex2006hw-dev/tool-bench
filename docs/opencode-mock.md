# opencode mock (replica service)

Deterministic world mock for the opencode AI models MCP server
(`gateway/opencode_mcp.py`), captured in
`servers/snapshots/opencode.json` (47 tools).

- Crate: `crates/services/opencode/` (`mock-opencode`, PORT 8131)
- Catalog: service `opencode` (`mode = "replica"`), endpoint `opencode`
  at `mcp_port = 7133`
- Tools served verbatim from the snapshot; `tools/call` dispatches
  canned echoes:
  - `opencode_list_models` — searchable embedded inventory (query,
    free_only, type filters)
  - `opencode_chat` — generic chat by `model` ref
  - `opencode_default_model` — `opencode-go/beam-free`
  - `opencode_get_model` — entry detail (name/type/free)
  - `chat_opencode_go_beam_free` … — one bound tool per live model
    (43 tools, canonical refs embedded from live capture)

No live LLM and no network: every chat reply is
`[mock opencode] <ref> echoes: <prompt>`, so bench runs are fully
deterministic and quota-free.
