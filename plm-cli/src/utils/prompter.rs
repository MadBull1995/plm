extern crate colored;
use log::{debug, error, log_enabled, info, Level, warn};
use colored::*;
use dialoguer::theme::ColorfulTheme;

pub enum MessageType {
    Task,
    Warning,
    Info,
    Normal,
    Error,
    Verbose,
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
            MessageType::Task => info!(
                "{}",
                format!("[{}/{}] {}", self.step, self.total, self.text).cyan()
            ),
            MessageType::Error => error!("{}", format!("> [ERROR]: {}", self.text).red()),
            MessageType::Warning => warn!("{}", format!("> [WARN]: {}", self.text).yellow()),
            MessageType::Info => info!("{}", format!("> [INFO]: {}", self.text).bold()),
            MessageType::Normal => info!("{}", self.text),
            MessageType::Verbose => debug!("{}", format!("> [DEBUG]: {}", self.text).dimmed()),
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

    pub fn verbose(text: &str) {
        Prompter {
            step: 0,
            total: 0,
            text: text.to_string(),
            message_type: MessageType::Verbose,
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

pub fn plm_theme() -> ColorfulTheme {
    let mut theme = ColorfulTheme::default();
    theme
}
