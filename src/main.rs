use chrono::{Duration, Local, NaiveDateTime};

#[derive(Debug)]
pub struct Activity {
    pub project: String,
    pub description: String,
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
}

impl Activity {
    pub fn start(project: String, description: String, time: Option<NaiveDateTime>) -> Activity {
        Activity {
            project,
            description,
            start: time.unwrap_or_else(|| Local::now().naive_local()),
            end: None,
        }
    }

    pub fn stop(&mut self, time: Option<NaiveDateTime>) {
        self.end = time.or_else(|| Some(Local::now().naive_local()));
    }

    pub fn get_duration(&self) -> Duration {
        if let Some(end) = self.end {
            end.signed_duration_since(self.start)
        } else {
            Local::now().naive_local().signed_duration_since(self.start)
        }
    }
}

fn main() {
    println!("{:?}", Local::now().naive_local());
    let sample_activity = Activity::start(
        String::from("M22405"),
        String::from("Pakenham Station"),
        None,
    );

    println!("{:?}", sample_activity);
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("{:?}", sample_activity.get_duration());
}
