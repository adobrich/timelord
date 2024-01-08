use iced::{
    alignment, executor,
    widget::{canvas::Fill, container, keyed_column, scrollable, text, text_input, Column},
    window, Application, Color, Command, Element, Event, Length, Settings, Theme,
};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

mod error;
mod icons;
mod task;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);
static EDIT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

fn main() -> iced::Result {
    AppState::run(Settings {
        fonts: vec![include_bytes!("../fonts/typicons.ttf").as_slice().into()],
        window: window::Settings {
            exit_on_close_request: false,
            size: iced::Size::new(400.0, 500.0),
            min_size: Some(iced::Size::new(400.0, 400.0)),
            ..window::Settings::default()
        },
        antialiasing: true,
        ..Settings::default()
    })
    // -------------------------------------------------------
    // let mut a_new_task = task::Task::new("a new task", false);
    // a_new_task.start();
    // // std::thread::sleep(std::time::Duration::from_secs(2));
    // a_new_task.stop();
    // // std::thread::sleep(std::time::Duration::from_secs(2));
    // println!("{a_new_task:#?}");
    // -------------------------------------------------------
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    tasks: Vec<task::Task>,
}

#[derive(Debug)]
enum AppState {
    Loading,
    Loaded(RuntimeState),
}

#[derive(Debug, Default)]
struct RuntimeState {
    input_value: String,
    tasks: Vec<task::Task>,
    is_dirty: bool,
    is_saving: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, error::LoadError>),
    Saved(Result<(), error::SaveError>),
    InputChanged(String),
    CreateTask(bool),
    TaskMessage(usize, task::TaskMessage),
    EventOccurred(Event),
    Tick,
}

impl Application for AppState {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            AppState::Loading,
            Command::perform(SavedState::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from(PKG_NAME)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            AppState::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = AppState::Loaded(RuntimeState {
                            tasks: state.tasks,
                            ..Default::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        eprintln!("No Database found. Creating a new one.");
                        *self = AppState::Loaded(RuntimeState::default());
                    }
                    _ => {}
                }
                text_input::focus(INPUT_ID.clone())
            }
            AppState::Loaded(state) => {
                let mut saved = false;

                let command = match message {
                    Message::EventOccurred(event) => {
                        if let Event::Window(_, window::Event::CloseRequested) = event {
                            for f in state.tasks.iter_mut() {
                                f.stop();
                            }
                            let save = Command::perform(
                                SavedState {
                                    tasks: state.tasks.clone(),
                                }
                                .save(),
                                Message::Saved,
                            );
                            Command::batch(vec![save, iced::window::close(iced::window::Id::MAIN)])
                        } else {
                            Command::none()
                        }
                        // Command::batch(vec![save, window::close(iced::window::Id::MAIN)])

                        //     {
                        //         println!("{0:#?}", state.tasks);
                        //         let _ = Command::perform(
                        //             SavedState {
                        //                 tasks: state.tasks.clone(),
                        //             }
                        //             .save(),
                        //             Message::Saved,
                        //         );
                        //     }
                        //     window::close(id)
                        // } else {
                        //     Command::none()
                        // }
                    }
                    Message::Tick => {
                        // TODO: keep track of seconds between saves and update UI every 500 milis?
                        Command::none()
                    }
                    Message::InputChanged(value) => {
                        state.input_value = value;

                        Command::none()
                    }
                    Message::CreateTask(is_active) => {
                        if !state.input_value.is_empty() {
                            state
                                .tasks
                                .push(task::Task::new(state.input_value.clone(), is_active));
                            state.input_value.clear()
                        }
                        Command::none()
                    }
                    Message::Saved(_) => {
                        state.is_saving = false;
                        saved = true;

                        Command::none()
                    }
                    Message::TaskMessage(i, task::TaskMessage::Start) => {
                        // TODO: think about this one
                        for t in state.tasks.iter_mut() {
                            t.stop()
                        }
                        if let Some(task) = state.tasks.get_mut(i) {
                            task.start()
                        }

                        Command::none()
                    }
                    Message::TaskMessage(i, task::TaskMessage::Stop) => {
                        if let Some(task) = state.tasks.get_mut(i) {
                            task.stop()
                        }

                        Command::none()
                    }
                    Message::TaskMessage(i, task::TaskMessage::Delete) => {
                        state.tasks.remove(i);

                        Command::none()
                    }
                    Message::TaskMessage(i, task::TaskMessage::Edit) => {
                        for t in state.tasks.iter_mut() {
                            t.state = task::TaskState::Idle;
                        }

                        if let Some(task) = state.tasks.get_mut(i) {
                            task.state = task::TaskState::Editing
                        };

                        text_input::focus(EDIT_ID.clone())
                    }
                    Message::TaskMessage(i, task::TaskMessage::NameEdited(value)) => {
                        if let Some(task) = state.tasks.get_mut(i) {
                            task.name = value;
                        };

                        Command::none()
                    }
                    Message::TaskMessage(i, task::TaskMessage::EditCompleted) => {
                        if let Some(task) = state.tasks.get_mut(i) {
                            task.previous_name = task.name.clone();
                            task.state = task::TaskState::Idle
                        };

                        Command::none()
                    }
                    Message::TaskMessage(i, task::TaskMessage::EditCanceled) => {
                        if let Some(task) = state.tasks.get_mut(i) {
                            task.name = task.previous_name.clone();
                            task.state = task::TaskState::Idle
                        };

                        Command::none()
                    }
                    _ => Command::none(),
                };

                if !saved {
                    state.is_dirty = true;
                }

                let save = if state.is_dirty && !state.is_saving {
                    state.is_dirty = false;
                    state.is_saving = true;

                    Command::perform(
                        SavedState {
                            tasks: state.tasks.clone(),
                        }
                        .save(),
                        Message::Saved,
                    )
                } else {
                    Command::none()
                };

                Command::batch(vec![save, command])
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        let handle_application_close_request = iced::event::listen().map(Message::EventOccurred);

        let tick = iced::time::every(std::time::Duration::from_millis(500)).map(|_| Message::Tick);

        iced::Subscription::batch(vec![tick, handle_application_close_request])
    }

    fn view(&self) -> Element<Message, iced::Renderer<Self::Theme>> {
        match self {
            AppState::Loading => loading_message(),
            AppState::Loaded(RuntimeState {
                input_value, tasks, ..
            }) => {
                let input = text_input("What's next?", input_value)
                    .id(INPUT_ID.clone())
                    .on_input(Message::InputChanged)
                    .on_submit(Message::CreateTask(false))
                    .padding(10)
                    .width(Length::Fill)
                    .size(20);

                let tasks: Element<_> = if tasks.len() > 0 {
                    keyed_column(tasks.iter().enumerate().map(|(i, task)| {
                        (
                            task.id,
                            task.view(i)
                                .map(move |message| Message::TaskMessage(i, message)),
                        )
                    }))
                    .spacing(10)
                    .into()
                } else {
                    empty_message("Nothing to do??  Get back to work!")
                };

                let content = Column::new()
                    .push(input)
                    .push(tasks)
                    .spacing(10)
                    .padding(10)
                    .max_width(600);

                scrollable(
                    container(content)
                        .width(Length::Fill)
                        .height(Length::Shrink)
                        .padding(5)
                        .align_x(alignment::Horizontal::Center),
                )
                .into()
            }
        }
    }
}

impl SavedState {
    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories_next::ProjectDirs::from("xyz", "dobrich", PKG_NAME)
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or_default()
        };

        path.push("database.json");

        path
    }

    async fn save(self) -> Result<(), error::SaveError> {
        use tokio::io::AsyncWriteExt;

        let json = serde_json::to_string_pretty(&self).map_err(|_| error::SaveError::Format)?;
        let path = Self::path();
        if let Some(dir) = path.parent() {
            tokio::fs::create_dir_all(dir)
                .await
                .map_err(|_| error::SaveError::File)?;
        }
        {
            let mut file = tokio::fs::File::create(path)
                .await
                .map_err(|_| error::SaveError::File)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| error::SaveError::Write)?;
        }

        // Save at most once every 2 seconds
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        Ok(())
    }

    async fn load() -> Result<SavedState, error::LoadError> {
        tokio::fs::read_to_string(Self::path())
            .await
            .map_err(|_| error::LoadError::File)
            .and_then(|contents| {
                serde_json::from_str(&contents).map_err(|_| error::LoadError::Format)
            })
    }
}

fn empty_message(message: &str) -> Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(alignment::Horizontal::Center)
            .style(Color::from([0.7, 0.7, 0.7])),
    )
    .width(Length::Fill)
    .height(200)
    .center_y()
    .into()
}

fn loading_message<'a>() -> Element<'a, Message> {
    container(
        text("Loading...")
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(alignment::Horizontal::Center)
            .style(Color::from([0.7, 0.7, 0.7])),
    )
    .width(Length::Fill)
    .height(200)
    .center_y()
    .into()
}
