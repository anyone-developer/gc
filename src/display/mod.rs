use prettytable::{color, format, Attr, Cell, Row, Table};

use crate::app::config::Command;

pub struct Display {}

impl Display {
    pub fn list_commands(commands: &Vec<Command>) {
        let mut table = Table::new();

        table.set_format(*format::consts::FORMAT_DEFAULT);

        table.set_titles(Row::new(vec![
            Cell::new("Name")
                .with_style(Attr::ForegroundColor(color::GREEN))
                .with_style(Attr::Bold),
            Cell::new("Type")
                .with_style(Attr::ForegroundColor(color::YELLOW))
                .with_style(Attr::Bold),
            Cell::new("Detail")
                .with_style(Attr::ForegroundColor(color::RED))
                .with_style(Attr::Bold),
            Cell::new("Prefix")
                .with_style(Attr::ForegroundColor(color::MAGENTA))
                .with_style(Attr::Bold),
            Cell::new("Suffix")
                .with_style(Attr::ForegroundColor(color::MAGENTA))
                .with_style(Attr::Bold),
        ]));

        for command in commands {
            let _type = format!("{:?}", &command.r#type);
            let prefix = command.prefix.as_ref().map(String::as_str).unwrap_or("");
            let suffix = command.suffix.as_ref().map(String::as_str).unwrap_or("");

            table.add_row(Row::new(vec![
                Cell::new(&command.name),
                Cell::new(&_type),
                Cell::new(&command.detail),
                Cell::new(prefix),
                Cell::new(suffix),
            ]));
        }

        table.printstd();
    }
}
