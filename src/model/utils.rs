extern crate chrono;
use std::sync::mpsc::Sender;
use self::chrono::DateTime;

// Callbacks for sending messages in channels.
pub trait CheckedSend<T: Clone> {
    fn checked_send(&self, message: T, callback: impl FnOnce(T)) -> ();
}

impl<T: Clone> CheckedSend<T> for Sender<T> {
    fn checked_send(&self, message: T, callback: impl FnOnce(T)) -> () {
        let error_message = message.clone();
        if self.send(message).is_err() { callback(error_message) }
    }
}

// Adding time to logs.
pub trait TimeLogged {
    fn time_logged(&self) -> String;
}

impl TimeLogged for String {
    fn time_logged(&self) -> String {
        use chrono::Utc;

        let now: DateTime<Utc> = Utc::now();
        return now.to_rfc3339() + self;
    }
}

// Logging
pub trait Logging {
    fn log(&self, message: String) -> ();
}

impl Logging for Sender<String> {
    fn log(&self, message: String) -> () {
        self.checked_send(message, |_| {} )
    }
}