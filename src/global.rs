use crate::app::{
    config::{AppConfig, AppConfigAction},
    vars::{AppVars, AppVarsAction},
};
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref APP_CONFIG: Mutex<AppConfig> = Mutex::new(AppConfig::new());
    pub static ref APP_VARS: Mutex<AppVars> = Mutex::new(AppVars::new());
}
