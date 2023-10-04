use iced::alignment::{self, Alignment};
use iced::font::{self, Font};
use iced::keyboard;
use iced::theme::{self, Theme};
use iced::widget::{
    self, button, checkbox, column, container, row, scrollable, text, text_input, Text,
};

use iced::window;
use iced::{Application, Element};
use iced::{Color, Command, Length, Settings, Subscription};

use chrono::{Duration, Local, NaiveDateTime};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

fn main() -> iced::Result {
    Timelord::run(Settings::default())
}

#[derive(Debug)]
enum Timelord {
    Loading,
    Loaded(State),
}

#[derive(Debug)]
struct State {
    input_value: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    InputChanged,
    FontLoaded(Result<(), font::Error>),
}

const ICONS: Font = Font::with_name("Timelord-Icons");

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(20)
        .horizontal_alignment(alignment::Horizontal::Center)
}

fn edit_icon() -> Text<'static> {
    icon('\u{e800}')
}

fn stopwatch_icon() -> Text<'static> {
    icon('\u{e801}')
}

fn clock_icon() -> Text<'static> {
    icon('\u{e808}')
}

fn left_arrow_icon() -> Text<'static> {
    icon('\u{e809}')
}

fn right_arrow_icon() -> Text<'static> {
    icon('\u{e80a}')
}

fn settings_icon() -> Text<'static> {
    icon('\u{e80b}')
}

fn right_chevron_icon() -> Text<'static> {
    icon('\u{f006}')
}

fn left_chevron_icon() -> Text<'static> {
    icon('\u{f007}')
}

fn resume_icon() -> Text<'static> {
    icon('\u{f00f}')
}

fn stop_icon() -> Text<'static> {
    icon('\u{F080}')
}

fn export_icon() -> Text<'static> {
    icon('\u{f081}')
}

fn delete_icon() -> Text<'static> {
    icon('\u{f083}')
}

fn calandar_icon() -> Text<'static> {
    icon('\u{f4c5}')
}

impl Application for Timelord {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Timelord, Command<Message>) {
        (
            Timelord::Loading,
            Command::batch(vec![
                font::load(include_bytes!("../fonts/timelord-icons.ttf").as_slice())
                    .map(Message::FontLoaded),
                Command::perform(SavedState::load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Timelord - Time Tracker")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged => todo!(),
            Message::FontLoaded(_) => todo!(),
        }
    }

    fn view(&self) -> Element<Message> {
        column![button(row![stop_icon(), "TEST"]).on_press(Message::InputChanged),]
            .padding(20)
            .align_items(Alignment::Center)
            .into()
    }
}
// use std::collections::HashMap;

// use chrono::{Duration, Local, NaiveDateTime};
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Project {
//     pub name: String,
//     pub tasks: Vec<Task>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Task {
//     pub name: String,
//     pub start: NaiveDateTime,
//     pub end: NaiveDateTime,
//     pub tags: Vec<String>,
// }

// impl Task {
//     pub fn new(name: String) -> Task {
//         let date_time = Local::now().naive_local();
//         Task {
//             name,
//             start: date_time,
//             end: date_time,
//             tags: Default::default(),
//         }
//     }
// }

// fn main() -> serde_json::Result<()> {
//     let mut project1 = Project {
//         name: "M23297 - Amazon".to_string(),
//         tasks: Vec::new(),
//     };

//     let mut project2 = Project {
//         name: "M22572 - Chadstone".to_string(),
//         tasks: Vec::new(),
//     };

//     project1
//         .tasks
//         .push(Task::new("Model Bumpout 01".to_string()));
//     project1
//         .tasks
//         .push(Task::new("Model Bumpout 02".to_string()));

//     project2
//         .tasks
//         .push(Task::new("Issue CPC for fab".to_string()));

//     let mut db = HashMap::new();
//     db.insert(project1.name.clone(), project1);
//     db.insert(project2.name.clone(), project2);
//     db.entry("M22572 - Chadstone".to_string())
//         .and_modify(|e| e.tasks.push(Task::new("Another task".to_string())));

//     db.entry("M23297 - Amazon".to_string()).and_modify(|e| {
//         e.tasks
//             .iter()
//             .filter(|t| t.name != "Model Bumpout 02".to_string()))
//     });
//     println!("{:#?}", db);
//     Ok(())
// }
// use chrono::{Duration, Local, NaiveDateTime};
// use serde::{Deserialize, Serialize};
// use serde_json::{Result, Value};
// use std::{collections::HashMap, str::FromStr};

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Project {
//     name: String,
//     tasks: Vec<Task>,
// }

// #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
// pub struct TaskDuration {
//     start: NaiveDateTime,
//     end: Option<NaiveDateTime>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Task {
//     pub name: String,
//     pub time: Vec<TaskDuration>,
//     // pub is_archived: bool,
//     // pub last_modified: NaiveDateTime,
// }

// fn main() -> Result<()> {
//     let data = r#"
//             {
//                 "name": "Project Title",
//                 "tasks": [
//                     {
//                         "name": "First task",
//                         "time": [
//                             {
//                                 "start": "2023-08-01T00:00:00.000",
//                                 "end": "2023-08-02T00:00:00.000"
//                             }
//                         ]
//                     },
//                     {
//                         "name": "Second task",
//                         "time": [
//                             {
//                                 "start": "2023-08-01T00:00:00.000"
//                             }
//                         ]
//                     }
//                 ]
//             }
//     "#;

//     // let projects: HashMap<String, Value> = serde_json::from_str(data)?;
//     // let projects: Vec<Project> = serde_json::from_str(data)?;
//     // println!("{:#?}", projects.keys());

//     let mut projects: HashMap<String, Vec<Task>> = HashMap::new();
//     let task = Task {
//         name: "first task".to_string(),
//         time: vec![TaskDuration {
//             start: NaiveDateTime::from_str("2023-08-01T01:00:00.000").unwrap(),
//             end: Some(NaiveDateTime::from_str("2023-08-01T03:00:00.000").unwrap()),
//         }],
//     };

//     projects.insert("first project".to_string(), vec![task]);

//     println!("{:#?}", &projects);

//     println!(
//         "hours spent on first task = {:?}",
//         projects
//             .get("first project")
//             .unwrap()
//             .first()
//             .unwrap()
//             .time
//             .first()
//             .unwrap()
//             .end
//             .unwrap()
//             .signed_duration_since(
//                 projects
//                     .get("first project")
//                     .unwrap()
//                     .first()
//                     .unwrap()
//                     .time
//                     .first()
//                     .unwrap()
//                     .start
//             )
//             .num_hours()
//     );

//     Ok(())
// }
