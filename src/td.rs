use core::{fmt, panic, str};
use serde::{de::Error, Deserialize, Serialize};

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub situation: Situation,
    pub creation_date: Date,
    pub completion_date: Date,
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

    pub fn pretty_print(&self) {
        println!(
            "id: {}\ntitle: {}\ncontent: {}\nsituation: {}\ncreation_date: {}\ncompletion_date: {}",
            self.id,
            self.title,
            self.content,
            self.situation,
            self.creation_date.get_formatted(),
            self.completion_date.get_formatted()
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
        creation_date: Date {
            day: 0,
            month: 0,
            year: 0,
            minute: 0,
            hour: 0,
        },
        completion_date: Date {
            day: 0,
            month: 0,
            year: 0,
            minute: 0,
            hour: 0,
        },
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
            _ => {}
        }
    }
    new_todo
}
