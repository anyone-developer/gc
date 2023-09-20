use crate::constants;
use crate::global;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use strum_macros::EnumString;

use super::vars::AppVarsAction;

#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
pub enum CommandType {
    String,
    Shell, // todo: new feature for future. execute pre-defined shell script.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub r#type: CommandType,
    pub name: String,
    pub detail: String,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

pub trait CommandAction {
    fn new(name: String, detail: String, prefix: Option<String>, suffix: Option<String>)
        -> Command;
}

impl CommandAction for Command {
    fn new(
        name: String,
        detail: String,
        prefix: Option<String>,
        suffix: Option<String>,
    ) -> Command {
        Command {
            r#type: CommandType::String,
            name,
            detail,
            prefix,
            suffix,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    app: String,
    version: String,
    source: String,
    commands: HashMap<String, Command>,
}

pub trait AppConfigAction {
    fn new() -> AppConfig;
    fn add_command(&mut self, command: Command);
    fn delete_command(&mut self, name: &str);
    fn list_commands(&self) -> Vec<Command>;
    fn get_command(&self, name: &str) -> &Command;
    fn serializer(&self) -> String;
    fn deserializer(&mut self, json: &mut str);
    fn init_config(&mut self);
    fn save_config(&self);
    fn resume_config(&mut self);
}

impl AppConfigAction for AppConfig {
    fn new() -> AppConfig {
        let app_config = AppConfig {
            app: constants::APP_SECTION.to_string(),
            version: constants::APP_VERSION.to_string(),
            source: constants::APP_GIT_SOURCE.to_string(),
            commands: HashMap::new(),
        };
        return app_config;
    }
    fn add_command(&mut self, command: Command) {
        let name = command.name.clone();
        self.commands.insert(name, command);
    }
    fn delete_command(&mut self, name: &str) {
        self.commands.remove(name);
    }
    fn list_commands(&self) -> Vec<Command> {
        return self.commands.values().cloned().collect();
    }
    fn get_command(&self, name: &str) -> &Command {
        return &self.commands[name];
    }
    fn serializer(&self) -> String {
        let app_config = self.clone();
        let json_output = serde_json::to_string_pretty::<AppConfig>(&app_config).unwrap();
        return json_output;
    }

    fn deserializer(&mut self, json: &mut str) {
        match serde_json::from_str::<AppConfig>(json) {
            Ok(app_config) => {
                //tool::debug_print(json);
                self.app = app_config.app;
                self.version = app_config.version;
                self.source = app_config.source;
                self.commands = app_config.commands;
            }
            Err(err) => {
                panic!("parse config string failed. {:?}", err);
            }
        }
    }

    fn init_config(&mut self) {
        let file_exist = { global::APP_VARS.lock().unwrap().get_file_exist() };
        match file_exist {
            true => self.resume_config(),
            false => self.save_config(),
        }
    }

    fn save_config(&self) {
        let file_path = { global::APP_VARS.lock().unwrap().get_file_path() };
        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(error) => {
                panic!("Failed to create file: {}", error);
            }
        };

        let config_str = self.serializer();

        if let Err(error) = file.write_all(config_str.as_bytes()) {
            panic!("Failed to write to file: {}", error);
        }
    }

    fn resume_config(&mut self) {
        let file_path = { global::APP_VARS.lock().unwrap().get_file_path() };

        let mut config_str = fs::read_to_string(file_path).expect("Failed to read file");

        self.deserializer(config_str.as_mut_str());
    }
}
