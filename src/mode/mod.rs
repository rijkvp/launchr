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
        Walk::new(home_dir)
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let path = entry.path();
                if !path.is_file() {
                    return None;
                }
                let name = path.file_name().and_then(|name| name.to_str());
                if let Some(name) = name {
                    if name.contains(input) {
                        return Some(path.display().to_string());
                    }
                }
                None
            })
            .take(10)
            .collect()
    }
}
