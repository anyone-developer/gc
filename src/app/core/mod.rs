use std::io::BufRead;
use std::process::Command as StdCommand;
use std::process::Stdio;

use crate::app::config::AppConfigAction;
use crate::display::Display;
use crate::global;
use crate::{app::config::CommandAction, constants};
use clap::{Arg, Command};

fn init_add_command() -> Command {
    return Command::new("add")
        .about(constants::GC_ADD_ABOUT)
        .long_about(constants::GC_ADD_LONG_ABOUT)
        .arg(
            Arg::new("prefix")
                .long("prefix")
                .long_help(constants::GC_PREFIX_HELP),
        )
        .arg(
            Arg::new("suffix")
                .long("suffix")
                .long_help(constants::GC_SUFFIX_HELP),
        )
        .arg(
            Arg::new("command")
                .long_help(constants::GC_COMMAND_HELP)
                .required(true),
        )
        .arg(
            Arg::new("detail")
                .long_help(constants::GC_DETAIL_HELP)
                .required(true),
        );
}

fn init_delete_command() -> Command {
    return Command::new("delete")
        .about(constants::GC_DELETE_ABOUT)
        .long_about(constants::GC_DELETE_LONG_ABOUT)
        .arg(
            Arg::new("command")
                .long_help(constants::GC_COMMAND_HELP)
                .required(true),
        );
}

fn init_list_command() -> Command {
    return Command::new("list")
        .about(constants::GC_DELETE_ABOUT)
        .long_about(constants::GC_DELETE_LONG_ABOUT);
}

fn init_run_command() -> Command {
    return Command::new("run")
        .about(constants::GC_RUN_ABOUT)
        .long_about(constants::GC_RUN_LONG_ABOUT)
        .arg(
            Arg::new("command")
                .long_help(constants::GC_COMMAND_HELP)
                .required(true),
        )
        .arg(
            Arg::new("params")
                .long_help(constants::GC_DETAIL_HELP)
                .value_delimiter(',')
                .required(true),
        );
}

fn init_gc_command() -> Command {
    return Command::new("gc")
        .version(constants::GC_APP_VERSION)
        .author(constants::GC_APP_AUTHOR)
        .about(constants::GC_APP_ABOUT)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .help_expected(true)
        .subcommand(init_add_command())
        .subcommand(init_delete_command())
        .subcommand(init_run_command())
        .subcommand(init_list_command());
}

fn replace_command_detail(original: &str, params: Vec<&String>) -> String {
    let mut result = original.to_string();

    for (index, param) in params.iter().enumerate() {
        let placeholder = format!("#{}", index);
        result = result.replace(&placeholder, param);
    }

    result
}

fn execute_command(command: String, prefix: &str, suffix: &str) -> Result<(), String> {
    let (tx, rx) = std::sync::mpsc::channel();

    let command_thread = std::thread::spawn(move || {
        let mut child = match StdCommand::new(get_shell_command())
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(_) => {
                let _ = tx.send(Err(String::from("Failed to execute command")));
                return;
            }
        };

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let stdout_reader = std::io::BufReader::new(stdout);
        let stderr_reader = std::io::BufReader::new(stderr);

        for line in stdout_reader.lines() {
            if let Ok(line) = line {
                if let Err(_) = tx.send(Ok(line)) {
                    break;
                }
            }
        }

        for line in stderr_reader.lines() {
            if let Ok(line) = line {
                if let Err(_) = tx.send(Err(line)) {
                    break;
                }
            }
        }

        let _ = child.wait();
    });

    while let Ok(result) = rx.recv() {
        match result {
            Ok(line) => println!("{}{}{}", prefix, line, suffix),
            Err(err) => {
                println!("{}", err);
                break;
            }
        }
    }

    let _ = command_thread.join();

    Ok(())
}

#[cfg(target_os = "macos")]
fn get_shell_command() -> &'static str {
    "sh"
}

#[cfg(target_os = "windows")]
fn get_shell_command() -> &'static str {
    "cmd"
}

#[cfg(target_os = "linux")]
fn get_shell_command() -> &'static str {
    "sh"
}

pub struct AppCommands;

pub trait AppCommandsAction {
    fn init_args_parser();
    fn handle_add_command(sub_matches: &clap::ArgMatches);
    fn handle_delete_command(sub_matches: &clap::ArgMatches);
    fn handle_list_command(sub_matches: &clap::ArgMatches);
    fn handle_run_command(sub_matches: &clap::ArgMatches);
}

impl AppCommandsAction for AppCommands {
    fn init_args_parser() {
        match init_gc_command().get_matches().subcommand() {
            Some(("add", sub_matches)) => Self::handle_add_command(sub_matches),
            Some(("delete", sub_matches)) => Self::handle_delete_command(sub_matches),
            Some(("run", sub_matches)) => Self::handle_run_command(sub_matches),
            Some(("list", sub_matches)) => Self::handle_list_command(sub_matches),
            _ => {}
        }
    }

    fn handle_add_command(sub_matches: &clap::ArgMatches) {
        let name = sub_matches
            .get_one::<String>("command")
            .unwrap()
            .to_string();
        let detail = sub_matches
            .get_one::<String>("detail")
            .unwrap()
            .trim_matches('\'')
            .trim_matches('"')
            .to_string();
        let prefix = sub_matches.get_one::<String>("prefix").map(|s| s.clone());
        let suffix = sub_matches.get_one::<String>("suffix").map(|s| s.clone());

        let command =
            <super::config::Command as CommandAction>::new(name.clone(), detail, prefix, suffix);
        {
            let mut app_config = global::APP_CONFIG.lock().unwrap();
            app_config.add_command(command);
            app_config.save_config();
        }
        println!("Add Command {:?} Done", &name);
    }

    fn handle_delete_command(sub_matches: &clap::ArgMatches) {
        let name = sub_matches
            .get_one::<String>("command")
            .unwrap()
            .to_string();
        {
            let mut app_config = global::APP_CONFIG.lock().unwrap();
            app_config.delete_command(&name);
            app_config.save_config();
        }
        println!("Delete Command {:?} Done", &name);
    }

    fn handle_list_command(_: &clap::ArgMatches) {
        {
            let app_config = global::APP_CONFIG.lock().unwrap();
            let commands = app_config.list_commands();
            Display::list_commands(&commands);
        }
    }

    fn handle_run_command(sub_matches: &clap::ArgMatches) {
        let command = sub_matches
            .get_one::<String>("command")
            .unwrap()
            .to_string();
        let params: Vec<_> = sub_matches.get_many::<String>("params").unwrap().collect();

        let app_config = global::APP_CONFIG.lock().unwrap();

        let command_obj = app_config.get_command(&command);

        let detail = &command_obj.detail;

        let _replaced = replace_command_detail(detail, params);

        let prefix = command_obj
            .prefix
            .as_ref()
            .map(String::as_str)
            .unwrap_or("");
        let suffix = command_obj
            .suffix
            .as_ref()
            .map(String::as_str)
            .unwrap_or("");

        let _ = execute_command(_replaced, &prefix, &suffix);
    }
}
