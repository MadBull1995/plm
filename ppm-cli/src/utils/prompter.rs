extern crate colored;

use colored::*;
use dialoguer::theme::ColorfulTheme;

pub enum MessageType {
    Task,
    Warning,
    Info,
    Normal,
    Error,
}

pub struct Prompter {
    step: usize,
    total: usize,
    text: String,
    message_type: MessageType,
}

impl Prompter {
    fn display(&self) {
        match self.message_type {
            MessageType::Task => println!("{}", format!("[{}/{}] {}", self.step, self.total, self.text).cyan()),
            MessageType::Error => println!("{}", format!("> [ERROR]: {}", self.text).red()),
            MessageType::Warning => println!("{}", format!("> [WARN]: {}", self.text).yellow()),
            MessageType::Info => println!("{}", format!("> [INFO]: {}", self.text).dimmed()),
            MessageType::Normal => println!("{}", self.text),
        }
    }

    pub fn task(step: usize, total: usize, text: &str) {
        Prompter {
            step,
            total,
            text: text.to_string(),
            message_type: MessageType::Task,
        }
        .display();
    }

    pub fn normal(text: &str) {
        Prompter {
            step: 0,
            total: 0,
            text: text.to_string(),
            message_type: MessageType::Normal,
        }
        .display();
    }

    pub fn warning(text: &str) {
        Prompter {
            step: 0,
            total: 0,
            text: text.to_string(),
            message_type: MessageType::Warning,
        }
        .display();
    }

    pub fn info(text: &str) {
        Prompter {
            step: 0,
            total: 0,
            text: text.to_string(),
            message_type: MessageType::Info,
        }
        .display();
    }
    
    pub fn error(text: &str) {
        Prompter {
            step: 0,
            total: 0,
            text: text.to_string(),
            message_type: MessageType::Error,
        }
        .display();
    }
}

pub fn ppm_theme() -> ColorfulTheme {
    let mut theme = ColorfulTheme::default();
    theme
}