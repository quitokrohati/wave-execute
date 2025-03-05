pub struct Config {
    pub window_size: (i32, i32),
    pub last_script_path: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            window_size: (800, 600),
            last_script_path: String::new(),
        }
    }

    pub fn load(&mut self) {
        // Load configuration from a file
    }

    pub fn save(&self) {
        // Save configuration to a file
    }
}