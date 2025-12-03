use {
    crate::{constants::SCILLA_CONFIG_RELATIVE_PATH, error::ScillaError},
    serde::{Deserialize, Serialize},
    solana_commitment_config::CommitmentLevel,
    std::{env::home_dir, fs, path::PathBuf},
};

pub fn scilla_config_path() -> PathBuf {
    let mut path = home_dir().expect("Error getting home path");
    path.push(SCILLA_CONFIG_RELATIVE_PATH);
    path
}

pub fn expand_tilde(path: &str) -> PathBuf {
    // on tomls, the ~ is not automatically expanded, so we need to do it manually
    if path.starts_with("~/")
        && let Some(home) = home_dir() {
            return path.replacen("~", &home.to_string_lossy(), 1).into();
        }
    path.into()
}

fn deserialize_path_with_tilde<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(expand_tilde(&s))
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ScillaConfig {
    pub rpc_url: String,
    pub commitment_level: CommitmentLevel,
    #[serde(deserialize_with = "deserialize_path_with_tilde")]
    pub keypair_path: PathBuf,
}

impl ScillaConfig {
    pub fn load() -> Result<ScillaConfig, ScillaError> {
        let scilla_config_path = scilla_config_path();
        println!("Config Path Founded! Using {scilla_config_path:?}");
        if !scilla_config_path.exists() {
            return Err(ScillaError::ConfigPathDoesntExists);
        }
        let data = fs::read_to_string(scilla_config_path)?;
        let config: ScillaConfig = toml::from_str(&data)?;
        Ok(config)
    }
}
