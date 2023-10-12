use iced::alignment::{self, Alignment};
use iced::font::{self, Font};
use iced::keyboard;
use iced::theme::{self, Theme};
use iced::widget::{
    self, button, checkbox, column, container, keyed_column, row, scrollable, text, text_input,
    Text,
};

use iced::window;
use iced::{Application, Element};
use iced::{Color, Command, Length, Settings, Subscription};

use chrono::{Duration, Local, NaiveDateTime};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod icons;

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

#[derive(Debug, Default)]
struct State {
    input_value: String,
    filter: Filter,
    tasks: Vec<Task>,
    dirty: bool,
    saving: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    FontLoaded(Result<(), font::Error>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    CreateTask,
    FilterChanged(Filter),
    TaskMessage(usize, TaskMessage),
    TabPressed { shift: bool },
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
        let dirty = match self {
            Timelord::Loading => false,
            Timelord::Loaded(state) => state.dirty,
        };
        format!("Timelord{} - Time Tracker", if dirty { "*" } else { "" })
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Timelord::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = Timelord::Loaded(State {
                            input_value: state.input_value,
                            filter: state.filter,
                            tasks: state.tasks,
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
                let mut saved = false;

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
                        Command::none()
                    }

                    Message::FilterChanged(filter) => {
                        state.filter = filter;

                        Command::none()
                    }

                    Message::TaskMessage(i, TaskMessage::Delete) => {
                        state.tasks.remove(i);

                        Command::none()
                    }

                    Message::TaskMessage(i, task_message) => {
                        if let Some(task) = state.tasks.get_mut(i) {
                            let should_focus = matches!(task_message, TaskMessage::Edit);

                            task.update(task_message);

                            if should_focus {
                                let id = Task::text_input_id(i);
                                Command::batch(vec![
                                    text_input::focus(id.clone()),
                                    text_input::select_all(id),
                                ])
                            } else {
                                Command::none()
                            }
                        } else {
                            Command::none()
                        }
                    }

                    Message::Saved(_) => {
                        state.saving = false;
                        saved = true;

                        Command::none()
                    }

                    Message::TabPressed { shift } => {
                        if shift {
                            widget::focus_previous()
                        } else {
                            widget::focus_next()
                        }
                    }

                    _ => Command::none(),
                };

                if !saved {
                    state.dirty = true;
                }

                let save = if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(
                        SavedState {
                            input_value: state.input_value.clone(),
                            filter: state.filter,
                            tasks: state.tasks.clone(),
                        }
                        .save(),
                        Message::Saved,
                    )
                } else {
                    Command::none()
                };

                Command::batch(vec![command, save])
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            Timelord::Loading => loading_message(),
            Timelord::Loaded(State {
                input_value,
                filter,
                tasks,
                ..
            }) => {
                let header = row![
                    icons::calendar(),
                    text(Local::now().naive_local().format(" %A %d %b %Y"))
                        .width(Length::Fill)
                        .size(20)
                        .style(Color::from([0.5, 0.5, 0.5]))
                        .horizontal_alignment(alignment::Horizontal::Center)
                ]
                .padding(5);

                let input = text_input("group:task", input_value)
                    .id(INPUT_ID.clone())
                    .on_input(Message::InputChanged)
                    .on_submit(Message::CreateTask)
                    .padding(5)
                    .size(20);

                let todays_tasks = tasks.iter().filter(|task| filter.matches(task));
                let tasks: Element<_> = if todays_tasks.count() > 0 {
                    keyed_column(
                        tasks
                            .iter()
                            .enumerate()
                            .filter(|(_, task)| filter.matches(task))
                            .map(|(i, task)| {
                                (
                                    task.id,
                                    task.view(i)
                                        .map(move |message| Message::TaskMessage(i, message)),
                                )
                            }),
                    )
                    .spacing(10)
                    .into()
                } else {
                    empty_message(match filter {
                        _ => "You have not created a task yet...",
                    })
                };

                let content = column![header, input, tasks]
                    .spacing(10)
                    .padding(10)
                    .max_width(800);

                scrollable(container(content).width(Length::Fill).padding(5).center_x()).into()
            }
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        keyboard::on_key_press(|key_code, modifiers| match (key_code, modifiers) {
            (keyboard::KeyCode::Tab, _) => Some(Message::TabPressed {
                shift: modifiers.shift(),
            }),
            _ => None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    name: String,
    group: Option<String>,
    start: NaiveDateTime,
    end: NaiveDateTime,

    #[serde(skip)]
    state: TaskState,
}

#[derive(Debug, Default, Clone)]
pub enum TaskState {
    #[default]
    Idle,
    Editing,
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    Edit,
    DescriptionEdited(String),
    FinishedEdition,
    Delete,
}

impl Task {
    fn new(name: String) -> Task {
        let current_timestamp = Local::now().naive_local();
        match name.split_once(':') {
            Some((group, name)) => Task {
                id: Uuid::new_v4(),
                name: name.trim().to_string(),
                group: if group.len() > 0 {
                    Some(group.trim().to_string())
                } else {
                    None
                },
                start: current_timestamp,
                end: current_timestamp,
                state: TaskState::Idle,
            },
            None => Task {
                id: Uuid::new_v4(),
                name,
                group: None,
                start: current_timestamp,
                end: current_timestamp,
                state: TaskState::Idle,
            },
        }
    }

    fn text_input_id(i: usize) -> text_input::Id {
        text_input::Id::new(format!("task - {i}"))
    }

    fn update(&mut self, message: TaskMessage) {
        match message {
            TaskMessage::Edit => {
                self.state = TaskState::Editing;
            }
            TaskMessage::DescriptionEdited(new_description) => {
                match new_description.split_once(':') {
                    Some((group, name)) => {
                        if group.len() > 0 {
                            self.group = Some(group.to_string());
                            self.name = name.to_string();
                        } else {
                            self.group = None;
                            self.name = name.to_string();
                        }
                    }
                    None => {
                        self.name = new_description;
                    }
                }
            }
            TaskMessage::FinishedEdition => {
                if !self.name.is_empty() {
                    self.state = TaskState::Idle;
                }
            }
            TaskMessage::Delete => {}
        }
    }

    fn view(&self, i: usize) -> Element<TaskMessage> {
        match &self.state {
            TaskState::Idle => {
                let label = text(format!(
                    "{} {}",
                    match &self.group {
                        Some(group) => &group,
                        None => "",
                    },
                    &self.name
                ))
                .width(Length::Fill);

                row![
                    label,
                    button(icons::edit())
                        .on_press(TaskMessage::Edit)
                        .padding(10)
                        .style(theme::Button::Text)
                ]
                .spacing(10)
                .align_items(Alignment::Center)
                .into()
            }
            TaskState::Editing => {
                let current = match &self.group {
                    Some(group) => {
                        format!("{group}:{}", &self.name)
                    }
                    None => self.name.clone(),
                };
                let text_input = text_input("group:task", &current)
                    .id(Self::text_input_id(i))
                    .on_input(TaskMessage::DescriptionEdited)
                    .on_submit(TaskMessage::FinishedEdition)
                    .padding(10);

                row![
                    text_input,
                    button(
                        row![icons::delete(), "Delete"]
                            .spacing(10)
                            .align_items(Alignment::Center)
                    )
                    .on_press(TaskMessage::Delete)
                    .padding(10)
                    .style(theme::Button::Destructive)
                ]
                .spacing(10)
                .align_items(Alignment::Center)
                .into()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Filter {
    #[default]
    Today,
    PreviousWeek,
    NextWeek,
}

impl Filter {
    fn matches(self, task: &Task) -> bool {
        match self {
            Filter::Today => {
                task.start.date() == Local::now().date_naive()
                    || task.end.date() == Local::now().date_naive()
            }
            Filter::PreviousWeek => todo!(),
            Filter::NextWeek => todo!(),
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

fn empty_message(message: &str) -> Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(25)
            .style(Color::from([0.7, 0.7, 0.7])),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    input_value: String,
    filter: Filter,
    tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
enum LoadError {
    File,
    Format,
}

#[derive(Debug, Clone)]
enum SaveError {
    File,
    Write,
    Format,
}

impl SavedState {
    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("xyz", "dobrich", "timelord")
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

    async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;

        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::Format)?;

        let path = Self::path();
        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::File)?;
        }
        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::File)?;
            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }
        async_std::task::sleep(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}
