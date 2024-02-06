use ignore::Walk;

pub trait Mode {
    fn name(&self) -> &str;
    fn run(&mut self, input: &str) -> Vec<String>;
}

pub struct FileMode;
impl Mode for FileMode {
    fn name(&self) -> &str {
        "Files"
    }

    fn run(&mut self, input: &str) -> Vec<String> {
        let home_dir = dirs::home_dir().unwrap();
        let mut results = Vec::new();
        for result in Walk::new(home_dir) {
            if let Ok(entry) = result {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }
                if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                    if name.contains(input) {
                        results.push(path.display().to_string());
                    }
                }
            }
        }
        results
    }
}
