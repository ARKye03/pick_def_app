use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub struct MimetypeManager {
    user_config_path: PathBuf,
    current_defaults: HashMap<String, String>,
}

impl MimetypeManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let home = std::env::var("HOME")?;
        let user_config_path = PathBuf::from(home).join(".config").join("mimeapps.list");

        let mut manager = Self {
            user_config_path,
            current_defaults: HashMap::new(),
        };

        manager.load_current_defaults()?;
        Ok(manager)
    }

    pub fn get_default_app(&self, mimetype: &str) -> Option<&String> {
        self.current_defaults.get(mimetype)
    }

    pub fn set_default_app(
        &mut self,
        mimetype: &str,
        desktop_file: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.current_defaults
            .insert(mimetype.to_string(), desktop_file.to_string());
        self.save_defaults()
    }

    pub fn get_all_mimetypes(&self) -> Vec<String> {
        self.current_defaults.keys().cloned().collect()
    }

    fn load_current_defaults(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.user_config_path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&self.user_config_path)?;
        let mut in_default_section = false;

        for line in content.lines() {
            let line = line.trim();

            if line == "[Default Applications]" {
                in_default_section = true;
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                in_default_section = false;
                continue;
            }

            if in_default_section && line.contains('=') {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    self.current_defaults
                        .insert(parts[0].trim().to_string(), parts[1].trim().to_string());
                }
            }
        }

        Ok(())
    }

    fn save_defaults(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure the directory exists
        if let Some(parent) = self.user_config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = fs::File::create(&self.user_config_path)?;

        writeln!(file, "[Default Applications]")?;
        for (mimetype, app) in &self.current_defaults {
            writeln!(file, "{}={}", mimetype, app)?;
        }

        Ok(())
    }
}
