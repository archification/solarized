use crossterm::style::{
    SetBackgroundColor, SetForegroundColor, ResetColor, Color,
    Attribute, SetAttribute,
};
use crossterm::{
    ExecutableCommand,
    cursor,
    terminal::{
        Clear,
        ClearType
    }
};
use rand::{Rng, thread_rng};
use std::env;
use std::io::stdout;

pub const BACK: Color = Color::Rgb { r:7, g:54, b:66 };
pub const VIOLET: Color = Color::Rgb { r:108, g:113, b:196 };
pub const BLUE: Color = Color::Rgb { r:38, g:139, b:210 };
pub const CYAN: Color = Color::Rgb { r:42, g:161, b:152 };
pub const GREEN: Color = Color::Rgb { r:133, g:153, b:0 };
pub const YELLOW: Color = Color::Rgb { r:181, g:137, b:0 };
pub const ORANGE: Color = Color::Rgb { r:203, g:75, b:22 };
pub const RED: Color = Color::Rgb { r:211, g:1, b:2 };
pub const MAGENTA: Color = Color::Rgb { r:211, g:54, b:130 };
pub const WHITE: Color = Color::Rgb { r:147, g:161, b:161 };
pub const GREY: Color = Color::Rgb { r:88, g:110, b:117 };
pub const BOLD: Attribute = Attribute::Bold;
pub const UNDERLINED: Attribute = Attribute::Underlined;
pub const ITALIC: Attribute = Attribute::Italic;

pub struct Argument {
    pub name: String,
    pub short: Option<String>,
    pub long: String,
    pub help: String,
    pub takes_value: bool,
}

pub struct ArgumentParser {
    args: Vec<Argument>,
}

impl ArgumentParser {
    pub fn new() -> Self {
        ArgumentParser { args: vec![] }
    }

    pub fn add_argument(&mut self, arg: Argument) {
        self.args.push(arg);
    }

    pub fn parse(&self) -> Result<ParseResult, ParseError> {
        let args: Vec<String> = env::args().collect();
        let mut parse_result = ParseResult::new();

        let mut i = 1;
        while i < args.len() {
            let arg = self.args.iter().find(|arg| arg.long == args[i] || arg.short.as_ref().map_or(false, |short| *short == args[i]));
            match arg {
                Some(arg) => {
                    if arg.takes_value {
                        if i + 1 < args.len() {
                            parse_result.add_value(arg.name.clone(), args[i + 1].clone());
                            i += 1;
                        } else {
                            return Err(ParseError::MissingValue(arg.name.clone()));
                        }
                    } else {
                        parse_result.add_flag(arg.name.clone());
                    }
                }
                None => {
                    return Err(ParseError::UnknownArgument(args[i].clone()));
                }
            }
            i += 1;
        }

        Ok(parse_result)
    }
}

pub struct ParseResult {
    values: std::collections::HashMap<String, String>,
    flags: std::collections::HashSet<String>,
}

impl ParseResult {
    fn new() -> Self {
        ParseResult {
            values: std::collections::HashMap::new(),
            flags: std::collections::HashSet::new(),
        }
    }

    fn add_value(&mut self, name: String, value: String) {
        self.values.insert(name, value);
    }

    fn add_flag(&mut self, name: String) {
        self.flags.insert(name);
    }

    pub fn get_value(&self, name: &str) -> Option<&str> {
        self.values.get(name).map(|s| s.as_str())
    }

    pub fn has_flag(&self, name: &str) -> bool {
        self.flags.contains(name)
    }
}

#[derive(Debug)]
pub enum ParseError {
    MissingValue(String),
    UnknownArgument(String),
}

pub enum PrintMode {
    NewLine,
    SameLine,
}

fn format_message(message_fragments: &[(&str, Color, Vec<Attribute>)]) -> String {
    let mut formatted_message = String::new();
    for (message, color, attributes) in message_fragments {
        let lines: Vec<&str> = message.split('\n').collect();
        for (i, line) in lines.iter().enumerate() {
            formatted_message += &SetBackgroundColor(BACK).to_string();
            formatted_message += &SetForegroundColor(*color).to_string();
            for attribute in attributes {
                formatted_message += &SetAttribute(*attribute).to_string();
            }
            formatted_message.push_str(line);
            formatted_message += &ResetColor.to_string();
            if i < lines.len() - 1 {
                formatted_message.push('\n');
            }
        }
    }
    formatted_message
}

fn print_formatted(message_fragments: &[(&str, Color, Vec<Attribute>)], mode: PrintMode) {
    let formatted_message = format_message(message_fragments);
    match mode {
        PrintMode::NewLine => {
            println!(
                "{}{}{}",
                SetBackgroundColor(BACK),
                formatted_message,
                ResetColor,
            );
        },
        PrintMode::SameLine => {
            print!(
                "{}{}{}",
                SetBackgroundColor(BACK),
                formatted_message,
                ResetColor,
            );
        },
    }
}

pub fn print_colored(message: &[&str], colors: &[Color], mode: PrintMode) {
    let fragments: Vec<_> = message.iter().enumerate()
        .map(|(i, &m)| (m, colors[i % colors.len()], vec![]))
        .collect();
    print_formatted(&fragments, mode);
}

pub fn print_fancy(message_fragments: &[(&str, Color, Vec<Attribute>)], mode: PrintMode) {
    print_formatted(message_fragments, mode);
}

pub fn clear() {
    stdout()
        .execute(Clear(ClearType::All)).unwrap()
        .execute(cursor::MoveTo(0, 0)).unwrap();
}

fn random_color() -> Color {
    let mut rng = thread_rng();
    Color::Rgb {
        r: rng.gen_range(0..255),
        g: rng.gen_range(0..255),
        b: rng.gen_range(0..255),
    }
}

pub fn print_random_colored(message: &str, mode: PrintMode) {
    let mut formatted_message = String::new();
    for ch in message.chars() {
        let fg = random_color();
        let bg = random_color();
        formatted_message += &SetBackgroundColor(bg).to_string();
        formatted_message += &SetForegroundColor(fg).to_string();
        formatted_message.push(ch);
        formatted_message += &ResetColor.to_string();
    }
    match mode {
        PrintMode::NewLine => {
            println!(
                "{}{}{}",
                SetBackgroundColor(BACK),
                formatted_message,
                ResetColor,
            );
        },
        PrintMode::SameLine => {
            print!(
                "{}{}{}",
                SetBackgroundColor(BACK),
                formatted_message,
                ResetColor,
            );
        },
    }
}

fn next_color(color: Color) -> Color {
    let step = 10;
    if let Color::Rgb { r, g, b } = color {
        let new_r = (r as u8).wrapping_add(step) % 255;
        let new_g = (g as u8).wrapping_add(step) % 255;
        let new_b = (b as u8).wrapping_add(step) % 255;
        Color::Rgb { r: new_r, g: new_g, b: new_b }
    } else {
        color
    }
}

pub fn print_hypno_colored(message: &str, mode: PrintMode) {
    let mut formatted_message = String::new();
    let mut fg = random_color();
    let mut bg = random_color();
    for (i, ch) in message.chars().enumerate() {
        if i % 1 == 0 {
            fg = next_color(fg);
            bg = next_color(bg);
        }
        formatted_message += &SetBackgroundColor(bg).to_string();
        formatted_message += &SetForegroundColor(fg).to_string();
        formatted_message.push(ch);
        formatted_message += &ResetColor.to_string();
    }
    match mode {
        PrintMode::NewLine => {
            println!(
                "{}",
                formatted_message,
            );
        },
        PrintMode::SameLine => {
            print!(
                "{}",
                formatted_message,
            );
        },
    }
}


