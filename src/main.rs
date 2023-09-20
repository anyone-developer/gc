use app::{
    config::AppConfigAction,
    core::{AppCommands, AppCommandsAction},
};

pub mod app;
pub mod constants;
pub mod display;
pub mod global;
pub mod tool;

fn main() {
    {
        global::APP_CONFIG.lock().unwrap().init_config()
    }
    <AppCommands as AppCommandsAction>::init_args_parser();
}
