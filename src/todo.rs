use core::{fmt, panic, str};
use serde::{de::Error, Deserialize, Serialize};
use std::time::SystemTime;

use crate::terminal::{self, Terminal};

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub situation: Situation,
    pub creation_date: String,
    pub completion_date: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Situation {
    Finished,
    Unfinished,
}

impl fmt::Display for Situation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Todo {
    pub fn change_title(&mut self, new_title: &str) {
        self.title = new_title.to_string();
    }

    pub fn change_content(&mut self, new_content: &str) {
        self.content = new_content.to_string();
    }

    pub fn change_situation(&mut self, str: &str) {
        match str {
            "Unfinished" => self.situation = Situation::Unfinished,
            "Finished" => self.situation = Situation::Finished,
            _ => panic!("invalid situation -> {}", str),
        }
    }

    pub fn update_date(&self) -> String {
        let sys_date = chrono::Local::now();
        let formatted = format!("{}", sys_date.format("%d-%m-%Y | %H:%M"));

        formatted
    }

    pub fn change_date(&mut self, date: &str) {
        self.creation_date = date.to_string();
    }

    pub fn pretty_print(&self, clear: bool) {
        if clear == true {
            Terminal::clear();
        }

        println!(
            "id: {}\ntitle: {}\ncontent: {}\nsituation: {}\ncreation_date: {}\ncompletion_date: {}",
            self.id,
            self.title,
            self.content,
            self.situation,
            self.creation_date,
            self.completion_date,
        );
    }
}

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Date {
    day: u32,
    month: u32,
    year: u32,
    minute: u8,
    hour: u8,
}

impl Date {
    pub fn get_formatted(&self) -> String {
        let formatted_date = format!(
            "{}-{}-{} {}:{}",
            self.day, self.month, self.year, self.hour, self.minute
        );

        formatted_date
    }
}

pub fn new_todo() -> Todo {
    Todo {
        id: 0,
        title: "debug_title".to_string(),
        content: "debug_content".to_string(),
        situation: Situation::Unfinished,
        creation_date: "".to_string(),
        completion_date: "".to_string(),
    }
}

pub fn record_to_todo(record: Vec<String>) -> Todo {
    let mut new_todo = new_todo();
    for i in 0..record.len() {
        match i {
            0 => new_todo.id = record[i].parse().unwrap(),
            1 => new_todo.change_title(&record[i]),
            2 => new_todo.change_content(&record[i]),
            3 => new_todo.change_situation(&record[i]),
            4 => new_todo.change_date(&record[i]),
            _ => {}
        }
    }
    new_todo
}
