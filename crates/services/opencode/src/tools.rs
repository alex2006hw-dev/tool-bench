//! Deterministic mock for the opencode models MCP snapshot
//! (`servers/snapshots/opencode.json`, 47 tools). Canned responses:
//! every chat tool echoes the caller's prompt plus the canonical model ref,
//! `opencode_list_models` returns the embedded inventory (release order).

use serde_json::{Value, json};

/// Canonical model refs with display names and free flags.
pub static MODELS: [(&str, &str, bool); 43] = [
    ("opencode/beam-free", "Beam Free", true),
    ("opencode/big-pickle", "Big Pickle", true),
    ("opencode/crouton-alpha-free", "Crouton Alpha Free", true),
    ("opencode/deepseek-v4.1-flash-free", "DeepSeek V4.1 Flash Free", true),
    ("opencode/flint-40b-free", "Flint 40B Free", true),
    ("opencode/glm-5.3-flash-free", "GLM-5.3-Flash Free", true),
    ("opencode-go/beam-free", "Beam Free", true),
    ("opencode-go/claude-haiku-5-5", "Claude Haiku 5.5", false),
    ("opencode-go/crouton-alpha-free", "Crouton Alpha Free", true),
    ("opencode-go/deepseek-v4.1-flash", "DeepSeek V4.1 Flash", false),
    ("opencode-go/deepseek-v4-pro", "DeepSeek V4 Pro", false),
    ("opencode-go/gemini-4-argon", "Gemini 4 Argon", false),
    ("opencode-go/glm-5.2", "GLM-5.2", false),
    ("opencode-go/glm-5.3", "GLM-5.3", false),
    ("opencode-go/glm-5.3-flash", "GLM-5.3-Flash", false),
    ("opencode-go/gpt-5.6-luna", "GPT-5.6 Luna", false),
    ("opencode-go/gpt-6-luna", "GPT-6 Luna", false),
    ("opencode-go/grok-4.7", "Grok 4.7", false),
    ("opencode-go/hy3", "Hy3", false),
    ("opencode-go/hy4", "Hy4", false),
    ("opencode-go/hy4-preview", "Hy4 preview", false),
    ("opencode-go/iceman-contributor-free", "Iceman Contributor Free", true),
    ("opencode-go/kimi-k2.7-code", "Kimi K2.7 Code", false),
    ("opencode-go/kimi-k3", "Kimi K3", false),
    ("opencode-go/kimi-k3.1-pro", "Kimi K3.1 Pro", false),
    ("opencode-go/longcat-2.0", "LongCat-2.0", false),
    ("opencode-go/mimo-v2.5", "MiMo V2.5", false),
    ("opencode-go/mimo-v2.5-pro", "MiMo V2.5 Pro", false),
    ("opencode-go/mimo-v2.6-flash", "MiMo-V2.6-Flash", false),
    ("opencode-go/mimo-v2.6-pro", "MiMo-V2.6-Pro", false),
    ("opencode-go/minimax-m3", "MiniMax-M3", false),
    ("opencode-go/minimax-m3.1-flash", "MiniMax M3.1 Flash", false),
    ("opencode-go/muse-spark-1.2-contributor", "Muse Spark 1.2 Contributor", false),
    ("opencode-go/muse-spark-1.3-contributor", "Muse Spark 1.3 Contributor", false),
    ("opencode-go/qwen3.7-plus", "Qwen3.7 Plus", false),
    ("opencode-go/qwen3.8-flash", "Qwen3.8 Flash", false),
    ("opencode-go/qwen3.8-max", "Qwen3.8 Max", false),
    ("opencode-go/step-5-preview", "Step 5 Preview", false),
    ("opencode/iceman-free", "Iceman Free", true),
    ("opencode/inkling-2-small-free", "Inkling 2 Small Free", true),
    ("opencode/mimo-v2.6-flash-free", "MiMo-V2.6-Flash Free", true),
    ("opencode/nemotron-3.5-lightning-free", "Nemotron 3.5 Lightning Free", true),
    ("opencode/nemotron-3.5-super-vl-free", "Nemotron 3.5 Super VL Free", true),
];

fn prompt_of(args: &Value) -> String {
    if let Some(p) = args.get("prompt").and_then(Value::as_str) {
        return p.to_owned();
    }
    match args.get("messages") {
        Some(Value::Array(msgs)) => msgs.iter().map(|m| {
            let role = m.get("role").and_then(Value::as_str).unwrap_or("user");
            let c = m.get("content").and_then(Value::as_str).unwrap_or("");
            format!("{role}: {c}")
        }).collect::<Vec<_>>().join("\n\n"),
        Some(other) => other.to_string(),
        None => String::new(),
    }
}

fn canned_chat(model: &str, args: &Value) -> String {
    format!("[mock opencode] {model} echoes: {}", prompt_of(args))
}

pub fn dispatch(name: &str, args: &Value) -> Option<Result<Value, String>> {
    match name {
        "opencode_list_models" => {
            let query = args.get("query").and_then(Value::as_str).unwrap_or("").to_lowercase();
            let free_only = args.get("free_only").and_then(Value::as_bool).unwrap_or(false);
            let items: Vec<Value> = MODELS.iter()
                .filter(|(_, _, f)| !free_only || *f)
                .filter(|(r, _, _)| query.is_empty() || r.to_lowercase().contains(&query))
                .map(|(r, n, f)| json!({"ref": r, "name": n, "type": "chat", "free": f}))
                .collect();
            Some(Ok(json!({"count": items.len(), "models": items})))
        }
        "opencode_chat" => {
            let model = args.get("model").and_then(Value::as_str).unwrap_or("");
            if model.is_empty() { return Some(Err("opencode_chat requires `model`".into())); }
            Some(Ok(Value::String(canned_chat(model, args))))
        }
        "opencode_default_model" => {
            let default = MODELS.iter().find(|(r, _, _)| *r == "opencode-go/beam-free")
                .map(|(r, _, _)| *r).unwrap_or("opencode-go/beam-free");
            Some(Ok(json!({"default": default})))
        }
        "opencode_get_model" => {
            let wanted = args.get("model").and_then(Value::as_str).unwrap_or("");
            match MODELS.iter().find(|(r, _, _)| *r == wanted) {
                Some((r, n, f)) => Some(Ok(json!({"ref": r, "name": n, "type": "chat", "free": f}))),
                None => Some(Err(format!("model not found: {wanted}"))),
            }
        }
        "chat_opencode_beam_free" => Some(Ok(Value::String(canned_chat("opencode/beam-free", args)))),
        "chat_opencode_big_pickle" => Some(Ok(Value::String(canned_chat("opencode/big-pickle", args)))),
        "chat_opencode_crouton_alpha_free" => Some(Ok(Value::String(canned_chat("opencode/crouton-alpha-free", args)))),
        "chat_opencode_deepseek_v4_1_flash_free" => Some(Ok(Value::String(canned_chat("opencode/deepseek-v4.1-flash-free", args)))),
        "chat_opencode_flint_40b_free" => Some(Ok(Value::String(canned_chat("opencode/flint-40b-free", args)))),
        "chat_opencode_glm_5_3_flash_free" => Some(Ok(Value::String(canned_chat("opencode/glm-5.3-flash-free", args)))),
        "chat_opencode_go_beam_free" => Some(Ok(Value::String(canned_chat("opencode-go/beam-free", args)))),
        "chat_opencode_go_claude_haiku_5_5" => Some(Ok(Value::String(canned_chat("opencode-go/claude-haiku-5-5", args)))),
        "chat_opencode_go_crouton_alpha_free" => Some(Ok(Value::String(canned_chat("opencode-go/crouton-alpha-free", args)))),
        "chat_opencode_go_deepseek_v4_1_flash" => Some(Ok(Value::String(canned_chat("opencode-go/deepseek-v4.1-flash", args)))),
        "chat_opencode_go_deepseek_v4_pro" => Some(Ok(Value::String(canned_chat("opencode-go/deepseek-v4-pro", args)))),
        "chat_opencode_go_gemini_4_argon" => Some(Ok(Value::String(canned_chat("opencode-go/gemini-4-argon", args)))),
        "chat_opencode_go_glm_5_2" => Some(Ok(Value::String(canned_chat("opencode-go/glm-5.2", args)))),
        "chat_opencode_go_glm_5_3" => Some(Ok(Value::String(canned_chat("opencode-go/glm-5.3", args)))),
        "chat_opencode_go_glm_5_3_flash" => Some(Ok(Value::String(canned_chat("opencode-go/glm-5.3-flash", args)))),
        "chat_opencode_go_gpt_5_6_luna" => Some(Ok(Value::String(canned_chat("opencode-go/gpt-5.6-luna", args)))),
        "chat_opencode_go_gpt_6_luna" => Some(Ok(Value::String(canned_chat("opencode-go/gpt-6-luna", args)))),
        "chat_opencode_go_grok_4_7" => Some(Ok(Value::String(canned_chat("opencode-go/grok-4.7", args)))),
        "chat_opencode_go_hy3" => Some(Ok(Value::String(canned_chat("opencode-go/hy3", args)))),
        "chat_opencode_go_hy4" => Some(Ok(Value::String(canned_chat("opencode-go/hy4", args)))),
        "chat_opencode_go_hy4_preview" => Some(Ok(Value::String(canned_chat("opencode-go/hy4-preview", args)))),
        "chat_opencode_go_iceman_contributor_free" => Some(Ok(Value::String(canned_chat("opencode-go/iceman-contributor-free", args)))),
        "chat_opencode_go_kimi_k2_7_code" => Some(Ok(Value::String(canned_chat("opencode-go/kimi-k2.7-code", args)))),
        "chat_opencode_go_kimi_k3" => Some(Ok(Value::String(canned_chat("opencode-go/kimi-k3", args)))),
        "chat_opencode_go_kimi_k3_1_pro" => Some(Ok(Value::String(canned_chat("opencode-go/kimi-k3.1-pro", args)))),
        "chat_opencode_go_longcat_2_0" => Some(Ok(Value::String(canned_chat("opencode-go/longcat-2.0", args)))),
        "chat_opencode_go_mimo_v2_5" => Some(Ok(Value::String(canned_chat("opencode-go/mimo-v2.5", args)))),
        "chat_opencode_go_mimo_v2_5_pro" => Some(Ok(Value::String(canned_chat("opencode-go/mimo-v2.5-pro", args)))),
        "chat_opencode_go_mimo_v2_6_flash" => Some(Ok(Value::String(canned_chat("opencode-go/mimo-v2.6-flash", args)))),
        "chat_opencode_go_mimo_v2_6_pro" => Some(Ok(Value::String(canned_chat("opencode-go/mimo-v2.6-pro", args)))),
        "chat_opencode_go_minimax_m3" => Some(Ok(Value::String(canned_chat("opencode-go/minimax-m3", args)))),
        "chat_opencode_go_minimax_m3_1_flash" => Some(Ok(Value::String(canned_chat("opencode-go/minimax-m3.1-flash", args)))),
        "chat_opencode_go_muse_spark_1_2_contributor" => Some(Ok(Value::String(canned_chat("opencode-go/muse-spark-1.2-contributor", args)))),
        "chat_opencode_go_muse_spark_1_3_contributor" => Some(Ok(Value::String(canned_chat("opencode-go/muse-spark-1.3-contributor", args)))),
        "chat_opencode_go_qwen3_7_plus" => Some(Ok(Value::String(canned_chat("opencode-go/qwen3.7-plus", args)))),
        "chat_opencode_go_qwen3_8_flash" => Some(Ok(Value::String(canned_chat("opencode-go/qwen3.8-flash", args)))),
        "chat_opencode_go_qwen3_8_max" => Some(Ok(Value::String(canned_chat("opencode-go/qwen3.8-max", args)))),
        "chat_opencode_go_step_5_preview" => Some(Ok(Value::String(canned_chat("opencode-go/step-5-preview", args)))),
        "chat_opencode_iceman_free" => Some(Ok(Value::String(canned_chat("opencode/iceman-free", args)))),
        "chat_opencode_inkling_2_small_free" => Some(Ok(Value::String(canned_chat("opencode/inkling-2-small-free", args)))),
        "chat_opencode_mimo_v2_6_flash_free" => Some(Ok(Value::String(canned_chat("opencode/mimo-v2.6-flash-free", args)))),
        "chat_opencode_nemotron_3_5_lightning_free" => Some(Ok(Value::String(canned_chat("opencode/nemotron-3.5-lightning-free", args)))),
        "chat_opencode_nemotron_3_5_super_vl_free" => Some(Ok(Value::String(canned_chat("opencode/nemotron-3.5-super-vl-free", args)))),
        _ => None,
    }
}
