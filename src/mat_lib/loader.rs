use std::fs;

pub fn load_file(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Err(e) => {
            let message = format!("Failed to load file {path}: {e}");
            Err(message)
        }
        Ok(content) => Ok(content),
    }
}
