use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Script {
    pub name: String,
    pub content: String,
}

pub fn load_script(file_path: &str) -> Result<Script, std::io::Error> {
    let content = std::fs::read_to_string(file_path)?;
    let script: Script = serde_json::from_str(&content)?;
    Ok(script)
}

pub fn save_script(file_path: &str, script: &Script) -> Result<(), std::io::Error> {
    let content = serde_json::to_string(script)?;
    std::fs::write(file_path, content)?;
    Ok(())
}