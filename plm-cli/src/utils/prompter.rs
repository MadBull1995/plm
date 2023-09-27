// Copyright 2023 Sylk Technologies
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate colored;
use colored::*;
use dialoguer::theme::ColorfulTheme;
use log::{debug, error, info, warn};

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
    ColorfulTheme::default()
}
