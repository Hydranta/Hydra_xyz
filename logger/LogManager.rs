use serde_json::{Value as JsonValue};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::error::Error;
use std::fs::{OpenOptions, File};
use std::io::Write;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Trace,
}

impl Level {
    pub fn from_u32(level: u32) -> Level {
        match level {
            0 => Level::Debug,
            1 => Level::Info,
            2 => Level::Warn,
            3 => Level::Error,
            4 => Level::Fatal,
            _ => Level::Trace,
        }
    }
}

pub trait Logger {
    fn log(&self, category: &str, level: Level, message: &str);
    fn set_pattern(&mut self, pattern: &str);
    fn disable_category(&mut self, category: &str);
}

pub struct ConsoleLogger {
    level: Level,
    pattern: String,
    disabled_categories: Vec<String>,
}

impl ConsoleLogger {
    pub fn new(level: Level) -> Self {
        ConsoleLogger {
            level,
            pattern: "%D %T %L [%C]: %M".to_string(),
            disabled_categories: Vec::new(),
        }
    }

    fn apply_pattern(&self, category: &str, level: Level, message: &str) -> String {
        self.pattern
            .replace("%D", "2024-12-30") // Example date
            .replace("%T", "12:00:00")  // Example time
            .replace("%L", &format!("{:?}", level))
            .replace("%C", category)
            .replace("%M", message)
    }
}

impl Logger for ConsoleLogger {
    fn log(&self, category: &str, level: Level, message: &str) {
        if !self.disabled_categories.contains(&category.to_string()) && level >= self.level {
            let formatted_message = self.apply_pattern(category, level, message);
            println!("{}", formatted_message);
        }
    }

    fn set_pattern(&mut self, pattern: &str) {
        self.pattern = pattern.to_string();
    }

    fn disable_category(&mut self, category: &str) {
        self.disabled_categories.push(category.to_string());
    }
}

pub struct FileLogger {
    level: Level,
    pattern: String,
    disabled_categories: Vec<String>,
    file_name: String,
}

impl FileLogger {
    pub fn new(level: Level, file_name: &str) -> Self {
        FileLogger {
            level,
            pattern: "%D %T %L [%C]: %M".to_string(),
            disabled_categories: Vec::new(),
            file_name: file_name.to_string(),
        }
    }

    pub fn init(&self) -> Result<(), Box<dyn Error>> {
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.file_name)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    fn apply_pattern(&self, category: &str, level: Level, message: &str) -> String {
        self.pattern
            .replace("%D", "2024-12-30") // Example date
            .replace("%T", "12:00:00")  // Example time
            .replace("%L", &format!("{:?}", level))
            .replace("%C", category)
            .replace("%M", message)
    }
}

impl Logger for FileLogger {
    fn log(&self, category: &str, level: Level, message: &str) {
        if !self.disabled_categories.contains(&category.to_string()) && level >= self.level {
            let formatted_message = self.apply_pattern(category, level, message);
            if let Ok(mut file) = OpenOptions::new().append(true).open(&self.file_name) {
                let _ = writeln!(file, "{}", formatted_message);
            }
        }
    }

    fn set_pattern(&mut self, pattern: &str) {
        self.pattern = pattern.to_string();
    }

    fn disable_category(&mut self, category: &str) {
        self.disabled_categories.push(category.to_string());
    }
}

pub struct LoggerManager {
    loggers: Vec<Box<dyn Logger>>,
    reconfigure_lock: Mutex<()>,
}

impl LoggerManager {
    pub fn new() -> Self {
        LoggerManager {
            loggers: Vec::new(),
            reconfigure_lock: Mutex::new(),
        }
    }

    pub fn configure(&mut self, val: &JsonValue) -> Result<(), Box<dyn Error>> {
        let _lock = self.reconfigure_lock.lock().unwrap();

        self.loggers.clear();

        let mut global_level = Level::Trace;
        if let Some(level) = val.get("globalLevel") {
            if let Some(level_int) = level.as_u64() {
                global_level = Level::from_u32(level_int as u32);
            } else {
                return Err("parameter globalLevel has wrong type".into());
            }
        }

        let mut global_disabled_categories = Vec::new();
        if let Some(disabled_categories) = val.get("globalDisabledCategories") {
            if let Some(categories) = disabled_categories.as_array() {
                for category in categories {
                    if let Some(cat) = category.as_str() {
                        global_disabled_categories.push(cat.to_string());
                    }
                }
            } else {
                return Err("parameter globalDisabledCategories has wrong type".into());
            }
        }

        if let Some(loggers) = val.get("loggers") {
            if let Some(loggers_list) = loggers.as_array() {
                for logger_config in loggers_list {
                    if let Some(logger_config_obj) = logger_config.as_object() {
                        let mut logger: Box<dyn Logger> = match logger_config_obj.get("type") {
                            Some(type_val) if type_val == "console" => {
                                let level = logger_config_obj.get("level")
                                    .and_then(|v| v.as_u64())
                                    .map_or(Level::Info, |v| Level::from_u32(v as u32));
                                Box::new(ConsoleLogger::new(level))
                            }
                            Some(type_val) if type_val == "file" => {
                                let level = logger_config_obj.get("level")
                                    .and_then(|v| v.as_u64())
                                    .map_or(Level::Info, |v| Level::from_u32(v as u32));

                                let file_name = logger_config_obj.get("filename")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();

                                let file_logger = FileLogger::new(level, &file_name);
                                Box::new(file_logger)
                            }
                            _ => return Err("Unknown logger type".into()),
                        };

                        if let Some(pattern) = logger_config_obj.get("pattern") {
                            if let Some(pattern_str) = pattern.as_str() {
                                logger.set_pattern(pattern_str);
                            }
                        }

                        if let Some(disabled_categories) = logger_config_obj.get("disabledCategories") {
                            if let Some(categories) = disabled_categories.as_array() {
                                for category in categories {
                                    if let Some(cat) = category.as_str() {
                                        logger.disable_category(cat);
                                    }
                                }
                            }
                        }

                        self.loggers.push(logger);
                    }
                }
            } else {
                return Err("loggers parameter has wrong type".into());
            }
        }

        if self.loggers.is_empty() {
            self.loggers.push(Box::new(ConsoleLogger::new(global_level)));
        }

        for category in global_disabled_categories {
            self.disable_category(&category);
        }

        Ok(())
    }

    pub fn disable_category(&self, category: &str) {
        for logger in &self.loggers {
            logger.disable_category(category);
        }
    }

    pub fn log(&self, category: &str, level: Level, message: &str) {
        for logger in &self.loggers {
            logger.log(category, level, message);
        }
    }
}

fn main() {
    let config = r#"
    {
        "globalLevel": 1,
        "globalDisabledCategories": ["network"],
        "loggers": [
            {
                "type": "console",
                "level": 0,
                "pattern": "%D %T %L [%C]: %M",
                "disabledCategories": ["ui"]
            },
            {
                "type": "file",
                "level": 1,
                "filename": "app.log",
                "pattern": "%D %T %L [%C]: %M",
                "disabledCategories": ["network"]
            }
        ]
    }
    "#;

    let val: JsonValue = serde_json::from_str(config).unwrap();
    let mut logger_manager = LoggerManager::new();

    if let Err(e) = logger_manager.configure(&val) {
        eprintln!("Error configuring logger manager: {}", e);
    } else {
        logger_manager.log("network", Level::Info, "Network started");
        logger_manager.log("ui", Level::Debug, "UI initialized");
    }
}
