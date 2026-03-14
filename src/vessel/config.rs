use serde::Deserialize;

use super::wiring::{CliBackend, ToolConfig, VesselConfig};

#[derive(Deserialize, Default)]
struct TomlConfig {
    session: Option<String>,
    backend: Option<String>,
    llama: Option<TomlLlama>,
    models: Option<TomlModels>,
    tools: Option<TomlTools>,
}

#[derive(Deserialize, Default)]
struct TomlLlama {
    base_url: Option<String>,
}

#[derive(Deserialize)]
struct TomlModels {
    mountain: Option<String>,
    desert: Option<String>,
    forest: Option<String>,
    utility: Option<String>,
}

#[derive(Deserialize)]
struct TomlSpringTools {
    allowed: Option<Vec<String>>,
    mcp_config: Option<String>,
}

#[derive(Deserialize)]
struct TomlTools {
    allowed: Option<Vec<String>>,
    mcp_config: Option<String>,
    mountain: Option<TomlSpringTools>,
    desert: Option<TomlSpringTools>,
    forest: Option<TomlSpringTools>,
    confluence: Option<TomlSpringTools>,
    #[serde(rename = "still-lake")]
    still_lake: Option<TomlSpringTools>,
    decomposer: Option<TomlSpringTools>,
}

impl TomlConfig {
    fn into_vessel_config(self) -> VesselConfig {
        let backend = match self.backend.as_deref() {
            Some("crush") => CliBackend::Crush,
            Some("llama") => {
                let llama = self.llama.unwrap_or_default();
                let base_url = llama
                    .base_url
                    .unwrap_or_else(|| "http://localhost:8080".into());
                let default_model = self
                    .models
                    .as_ref()
                    .and_then(|m| m.mountain.clone())
                    .unwrap_or_else(|| "qwen3.5-9b-q4_k_m".into());
                CliBackend::llama(base_url, default_model)
            }
            _ => CliBackend::Claude,
        };

        let mut config =
            VesselConfig::for_backend(self.session.unwrap_or_else(|| "tao-flow".into()), backend);

        if let Some(models) = self.models {
            if let Some(m) = models.mountain {
                config.mountain_model = m;
            }
            if let Some(m) = models.desert {
                config.desert_model = m;
            }
            if let Some(m) = models.forest {
                config.forest_model = m;
            }
            if let Some(m) = models.utility {
                config.utility_model = m;
            }
        }

        if let Some(tools) = self.tools {
            if tools.allowed.is_some() || tools.mcp_config.is_some() {
                config.default_tools = Some(ToolConfig {
                    allowed_tools: tools.allowed.unwrap_or_default(),
                    mcp_config: tools.mcp_config,
                });
            }

            let overrides: [(&str, Option<TomlSpringTools>); 6] = [
                ("mountain", tools.mountain),
                ("desert", tools.desert),
                ("forest", tools.forest),
                ("confluence", tools.confluence),
                ("still-lake", tools.still_lake),
                ("decomposer", tools.decomposer),
            ];
            for (name, spring_tools) in overrides {
                if let Some(st) = spring_tools {
                    config.spring_tools.insert(
                        name.into(),
                        ToolConfig {
                            allowed_tools: st.allowed.unwrap_or_default(),
                            mcp_config: st.mcp_config,
                        },
                    );
                }
            }
        }

        config
    }
}

pub fn load_config(path: &str) -> Result<VesselConfig, crate::error::FlowError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| crate::error::FlowError::ConfigError(format!("failed to read {path}: {e}")))?;
    let toml_config: TomlConfig = toml::from_str(&content).map_err(|e| {
        crate::error::FlowError::ConfigError(format!("failed to parse {path}: {e}"))
    })?;
    Ok(toml_config.into_vessel_config())
}

pub fn load_config_or_default() -> VesselConfig {
    match load_config("tao_flow.toml") {
        Ok(config) => config,
        Err(_) => VesselConfig::new("tao-flow"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_config_uses_defaults() {
        let toml: TomlConfig = toml::from_str("").unwrap();
        let config = toml.into_vessel_config();
        assert_eq!(config.session, "tao-flow");
        assert_eq!(config.mountain_model, "claude-sonnet-4-6");
    }

    #[test]
    fn session_override() {
        let toml: TomlConfig = toml::from_str(r#"session = "my-flow""#).unwrap();
        let config = toml.into_vessel_config();
        assert_eq!(config.session, "my-flow");
    }

    #[test]
    fn claude_backend_is_default() {
        let toml: TomlConfig = toml::from_str("").unwrap();
        let config = toml.into_vessel_config();
        assert_eq!(config.desert_model, "claude-sonnet-4-6");
    }

    #[test]
    fn crush_backend() {
        let toml: TomlConfig = toml::from_str(r#"backend = "crush""#).unwrap();
        let config = toml.into_vessel_config();
        assert_eq!(config.mountain_model, "anthropic/claude-sonnet-4-6");
    }

    #[test]
    fn llama_backend_with_defaults() {
        let toml: TomlConfig = toml::from_str(r#"backend = "llama""#).unwrap();
        let config = toml.into_vessel_config();
        assert_eq!(config.mountain_model, "qwen3.5-9b-q4_k_m");
    }

    #[test]
    fn model_overrides() {
        let input = r#"
[models]
mountain = "big-model"
desert = "fast-model"
"#;
        let toml: TomlConfig = toml::from_str(input).unwrap();
        let config = toml.into_vessel_config();
        assert_eq!(config.mountain_model, "big-model");
        assert_eq!(config.desert_model, "fast-model");
        assert_eq!(config.forest_model, "claude-sonnet-4-6");
    }

    #[test]
    fn tool_defaults() {
        let input = r#"
[tools]
allowed = ["WebSearch"]
mcp_config = "/tmp/mcp.json"
"#;
        let toml: TomlConfig = toml::from_str(input).unwrap();
        let config = toml.into_vessel_config();
        let tools = config.default_tools.unwrap();
        assert_eq!(tools.allowed_tools, vec!["WebSearch"]);
        assert_eq!(tools.mcp_config.unwrap(), "/tmp/mcp.json");
    }

    #[test]
    fn per_spring_tool_overrides() {
        let input = r#"
[tools]
allowed = ["WebSearch"]

[tools.mountain]
allowed = ["WebSearch", "WebFetch"]
"#;
        let toml: TomlConfig = toml::from_str(input).unwrap();
        let config = toml.into_vessel_config();
        let mountain = config.spring_tools.get("mountain").unwrap();
        assert_eq!(mountain.allowed_tools, vec!["WebSearch", "WebFetch"]);
    }

    #[test]
    fn missing_file_returns_error() {
        let result = load_config("/nonexistent/tao_flow.toml");
        assert!(result.is_err());
    }
}
