// 1. check .gc_config file
// 2. parse .gc_config file
// 3. read args.
// 4. matching config command
// 5. provide inline commands. eg. add, delete, run

// gc add fc "git remove $0 $1" --prefix + --surfix [x]
// gc delete fc
// gc run fc aa bb dd

// git diff --name-only $0 $1

// support $0, $1, ${PATH}

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
