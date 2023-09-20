use std::fs;

use dirs::home_dir;

use crate::constants;

pub struct AppVars {
    file_path: String,
    file_exist: bool,
}

impl AppVars {
    fn find_home_dir() -> String {
        if let Some(path) = home_dir() {
            return path.display().to_string();
        }
        panic!("Unable to determine home directory");
    }

    fn get_metadata(file_path: &str) -> bool {
        if let Ok(metadata) = fs::metadata(file_path) {
            if metadata.is_file() {
                return true;
            } else {
                panic!("{} is not a file.", constants::FILE_NAME);
            }
        }
        return false;
    }

    fn init_vars() -> (String, bool) {
        let path = Self::find_home_dir();
        let file_path = format!("{}/{}", path, constants::FILE_NAME).to_string();
        let file_exist = Self::get_metadata(&file_path);
        return (file_path, file_exist);
    }
}

pub trait AppVarsAction {
    fn new() -> AppVars;
    fn get_file_path(&self) -> String;
    fn get_file_exist(&self) -> bool;
}

impl AppVarsAction for AppVars {
    fn new() -> AppVars {
        let (file_path, file_exist) = Self::init_vars();
        return AppVars {
            file_path,
            file_exist,
        };
    }

    fn get_file_path(&self) -> String {
        return self.file_path.clone();
    }
    fn get_file_exist(&self) -> bool {
        return self.file_exist.clone();
    }
}
