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

pub enum PrintMode {
    NewLine,
    SameLine,
}

fn format_message(message_fragments: &[(&str, Color, Vec<Attribute>)]) -> String {
    let mut formatted_message = String::new();
    for (message, color, attributes) in message_fragments {
        let mut start_of_line = true;
        for ch in message.chars() {
            if start_of_line {
                formatted_message += &SetBackgroundColor(BACK).to_string();
                formatted_message += &SetForegroundColor(*color).to_string();
                for attribute in attributes {
                    formatted_message += &SetAttribute(*attribute).to_string();
                }
                start_of_line = false;
            }
            formatted_message.push(ch);
            if ch == '\n' {
                formatted_message += &ResetColor.to_string();
                start_of_line = true;
            }
        }
        if !message.ends_with('\n') {
            formatted_message.push('\n');  // Ensure newline at end if not present
        }
        formatted_message += &ResetColor.to_string(); // Ensure reset at end of each fragment
    }
    formatted_message
}

fn print_formatted(message_fragments: &[(&str, Color, Vec<Attribute>)], mode: PrintMode) {
    let formatted_message = format_message(message_fragments);
    match mode {
        PrintMode::NewLine => {
            println!("{}{}{}", SetBackgroundColor(BACK), formatted_message, ResetColor);
        },
        PrintMode::SameLine => {
            print!("{}{}{}", SetBackgroundColor(BACK), formatted_message, ResetColor);
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
