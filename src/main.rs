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
    Timelord::run(Settings {
        window: window::Settings {
            size: (500, 400),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug)]
enum Timelord {
    Loading,
    Loaded(State),
}

#[derive(Debug)]
struct Task {
    name: String,
    group: Option<String>,
    start: NaiveDateTime,
    end: NaiveDateTime,
}

impl Task {
    fn new(name: String) -> Task {
        let current_timestamp = Local::now().naive_local();
        match name.split_once(':') {
            Some((group, name)) => Task {
                name: name.trim().to_string(),
                group: Some(group.trim().to_string()),
                start: current_timestamp,
                end: current_timestamp,
            },
            None => Task {
                name,
                group: None,
                start: current_timestamp,
                end: current_timestamp,
            },
        }
    }
}

#[derive(Debug, Default)]
struct State {
    input_value: String,
    tasks: Vec<Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    input_value: String,
}

impl SavedState {
    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("xyz", "dobrich", "Timelord")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or_default()
        };

        path.push("timelord_db.json");
        path
    }

    async fn load() -> Result<SavedState, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();
        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::File)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::File)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }
}

#[derive(Debug, Clone)]
enum LoadError {
    File,
    Format,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    FontLoaded(Result<(), font::Error>),
    InputChanged(String),
    CreateTask,
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
                // icons: https://github.com/stephenhutchings/typicons.font
                font::load(include_bytes!("../fonts/typicons.ttf").as_slice())
                    .map(Message::FontLoaded),
                Command::perform(SavedState::load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Timelord - Time Tracker")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Timelord::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = Timelord::Loaded(State {
                            input_value: state.input_value,
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = Timelord::Loaded(State::default());
                    }
                    _ => {}
                }

                text_input::focus(INPUT_ID.clone())
            }
            Timelord::Loaded(state) => {
                // TODO: update title
                let mut _saved = false;

                let command = match message {
                    Message::InputChanged(value) => {
                        state.input_value = value;

                        Command::none()
                    }

                    Message::CreateTask => {
                        if !state.input_value.is_empty() {
                            state.tasks.push(Task::new(state.input_value.clone()));
                            state.input_value.clear();
                        }
                        dbg!(state);
                        Command::none()
                    }

                    _ => Command::none(),
                };

                Command::batch(vec![command])
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            Timelord::Loading => loading_message(),
            Timelord::Loaded(State { input_value, .. }) => column![
                row![
                    button(left_arrow_icon()),
                    text(Local::now().naive_local().format("Week %-V"))
                        .width(Length::Fill)
                        .horizontal_alignment(alignment::Horizontal::Center),
                    button(right_arrow_icon()),
                ]
                .height(50)
                .padding(10),
                text_input("group:task", input_value)
                    .id(INPUT_ID.clone())
                    .on_input(Message::InputChanged)
                    .on_submit(Message::CreateTask),
            ]
            .padding(20)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .into(),
        }
    }
}

fn loading_message<'a>() -> Element<'a, Message> {
    container(
        text("Loading...")
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}

const ICONS: Font = Font::with_name("typicons");

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(20)
        .horizontal_alignment(alignment::Horizontal::Center)
}

fn edit_icon() -> Text<'static> {
    icon('\u{E0C3}')
}

fn stopwatch_icon() -> Text<'static> {
    icon('\u{E10C}')
}

fn clock_icon() -> Text<'static> {
    icon('\u{E120}')
}

fn left_arrow_icon() -> Text<'static> {
    icon('\u{E00D}')
}

fn right_arrow_icon() -> Text<'static> {
    icon('\u{E01A}')
}

fn settings_icon() -> Text<'static> {
    icon('\u{E050}')
}

fn right_chevron_icon() -> Text<'static> {
    icon('\u{E049}')
}

fn left_chevron_icon() -> Text<'static> {
    icon('\u{E047}')
}

fn resume_icon() -> Text<'static> {
    icon('\u{E0B0}')
}

fn stop_icon() -> Text<'static> {
    icon('\u{E0B6}')
}

fn export_icon() -> Text<'static> {
    icon('\u{E06D}')
}

fn delete_icon() -> Text<'static> {
    icon('\u{E123}')
}

fn calandar_icon() -> Text<'static> {
    icon('\u{E039}')
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
