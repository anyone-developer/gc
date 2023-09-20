use std::process::Command as StdCommand;
use std::process::Output;
use std::sync::mpsc;
use std::thread;

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
        let placeholder = format!("${}", index);
        result = result.replace(&placeholder, param);
    }

    result
}

fn single_thread_execute_command(command: &str) -> Result<String, String> {
    let output: Output = match StdCommand::new("sh").arg("-c").arg(command).output() {
        Ok(output) => output,
        Err(_) => return Err(String::from("Failed to execute command")),
    };

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(stderr.to_string())
    }
}

fn execute_command(command: String) -> Result<(), String> {
    let (tx, rx) = mpsc::channel();

    // 创建一个线程执行命令并将每行的 stdout 发送到通道
    let command_thread = thread::spawn(move || {
        let output: Output = match StdCommand::new("sh").arg("-c").arg(command).output() {
            Ok(output) => output,
            Err(_) => {
                let _ = tx.send(Err(String::from("Failed to execute command")));
                return;
            }
        };

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if let Err(_) = tx.send(Ok(line.to_string())) {
                    break;
                }
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let _ = tx.send(Err(stderr.to_string()));
        }
    });

    // 在主线程中接收通道中的消息并进行输出
    while let Ok(result) = rx.recv() {
        match result {
            Ok(line) => println!("{}", line),
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }

    // 等待命令执行线程结束
    let _ = command_thread.join();

    Ok(())
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

        let _ = execute_command(_replaced);
    }
}
