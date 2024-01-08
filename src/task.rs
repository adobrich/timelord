use std::hash::BuildHasherDefault;

use chrono::{DateTime, Duration, DurationRound, NaiveDateTime, Utc};
use chrono_tz::Tz;
use iced::{
    widget::{button, horizontal_space, row, rule::FillMode, text, text_input},
    Element, Length,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::icons;
use crate::EDIT_ID;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub name: String,
    pub tag: Option<String>,
    // TODO: would be nicer if we didn't need this field... think about it a bit more.
    pub previous_name: String,
    pub hours: Vec<TaskHours>,
    pub is_active: bool,

    #[serde(skip)]
    pub state: TaskState,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TaskState {
    #[default]
    Idle,
    Editing,
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    Edit,
    NameEdited(String),
    EditCompleted,
    EditCanceled,
    Delete,
    Start,
    Stop,
}

impl Task {
    pub fn new<S: Into<String> + std::clone::Clone>(name: S, set_active: bool) -> Self {
        let n: String = name.clone().into();
        let stuff = n.split_once(':');
        Self {
            id: Uuid::new_v4(),
            // name: name.clone().into(),
            name: stuff.clone().unwrap_or_default().1.to_string(),
            tag: stuff.and(Some(stuff.unwrap().0.to_string())),
            previous_name: name.into(),
            hours: if set_active {
                vec![TaskHours::new()]
            } else {
                Vec::new()
            },
            is_active: set_active,
            state: TaskState::Idle,
        }
    }

    pub fn start(&mut self) {
        match self.is_active {
            true => eprintln!("Task already active"),
            false => {
                self.is_active = true;
                self.hours.push(TaskHours::new());
            }
        }
    }

    pub fn stop(&mut self) {
        match self.is_active {
            true => {
                self.is_active = false;
                self.hours.last_mut().map(|hours| {
                    hours.end = Some(Utc::now());
                    if let (Some(start), Some(end)) = (hours.start, hours.end) {
                        hours.duration = end.signed_duration_since(start);
                    } else {
                        // TODO: log? eprintln!("Failed to stop task. `Duration` not set.")
                    }
                });
            }
            false => {} // TODO: log? eprintln!("Task not active"),
        }
    }
    pub fn view(&self, i: usize) -> Element<TaskMessage> {
        match &self.state {
            TaskState::Idle => row![]
                .push(
                    button({
                        match &self.tag {
                            Some(tag) => self.name.split(':').last().unwrap_or_default(),
                            None => self.name.as_str(),
                        }
                    })
                    .width(Length::Fill)
                    .on_press(TaskMessage::Edit),
                )
                .push(horizontal_space(Length::Fixed(10.0)))
                .push(
                    text(
                        {
                            let duration = chrono::Duration::seconds(
                                self.hours
                                    .iter()
                                    .map(|h| h.duration.num_seconds())
                                    .collect::<Vec<i64>>()
                                    .iter()
                                    .sum::<i64>(),
                            );
                            let hours = &duration.num_hours();
                            let mins = (*&duration - (chrono::Duration::seconds(hours * 60)))
                                .num_minutes();
                            format!("{:0>2}:{:0>2}", hours, mins)
                        }, // .map(|d| {

                           //     format!(
                           //         "{:0>2}:{:0>2}",
                           //         chrono::Duration::seconds(d).num_hours(),
                           //         chrono::Duration::seconds(d)
                           //             .checked_sub(&chrono::Duration::seconds(
                           //                 chrono::Duration::seconds(d)
                           //                     - 60 * chrono::Duration::hours(d)
                           //             ))
                           //             .ok_or(chrono::Duration::zero())
                           //             .unwrap()
                           //             .num_seconds()
                           //     )
                           // }),
                    ), // self.hours
                       //     .iter()
                       //     .filter_map(|h| {
                       //         if h.start.unwrap_or_default().date_naive() == Utc::now().date_naive()
                       //             || h.end.unwrap_or_default().date_naive() == Utc::now().date_naive()
                       //         {
                       //             Some(&mut h.duration.num_seconds())
                       //         } else {
                       //             Some(0)
                       //         }
                       //     })
                       //     .collect::<i64>(),
                )
                .push(horizontal_space(Length::Fixed(10.0)))
                .push(
                    button(if self.is_active {
                        icons::stop()
                    } else {
                        icons::resume()
                    })
                    .on_press(if self.is_active {
                        TaskMessage::Stop
                    } else {
                        TaskMessage::Start
                    }),
                )
                .into(),
            TaskState::Editing => {
                //
                row![]
                    .push(
                        text_input(&self.name, &self.name)
                            .id(EDIT_ID.clone())
                            .on_input(TaskMessage::NameEdited)
                            .on_submit(TaskMessage::EditCompleted),
                    )
                    .push(horizontal_space(Length::Fill))
                    .push(button(icons::delete()).on_press(TaskMessage::Delete))
                    .push(row![
                        button(icons::cancel()).on_press(TaskMessage::EditCanceled)
                    ])
                    .into()
            }
        }
    }
}

#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TaskHours {
    // TODO: remove this when chrono `Duration` has serde support
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub duration: chrono::Duration,
    pub end: Option<DateTime<Utc>>,
    pub start: Option<DateTime<Utc>>,
}

impl Default for TaskHours {
    fn default() -> Self {
        Self {
            duration: chrono::Duration::zero(),
            end: None,
            start: None,
        }
    }
}

impl TaskHours {
    pub fn new() -> Self {
        let current_date_time = Utc::now();
        Self {
            start: Some(current_date_time),
            end: None,
            duration: chrono::Duration::zero(),
        }
    }

    // TODO: might not need thses
    pub fn start_with_tz(&self, tz: &str) -> Option<DateTime<chrono_tz::Tz>> {
        if let Ok(timezone) = tz.parse() {
            self.start
                .map(|start_time| start_time.with_timezone(&timezone))
        } else {
            None
        }
    }

    pub fn end_with_tz(&self, tz: &str) -> Option<DateTime<chrono_tz::Tz>> {
        if let Ok(timezone) = tz.parse() {
            self.end.map(|end_time| end_time.with_timezone(&timezone))
        } else {
            None
        }
    }
}
