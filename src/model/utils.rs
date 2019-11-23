use std::sync::mpsc::Sender;

pub trait CheckedSend<T: Clone> {
    fn checked_send(&self, message: T, callback: impl FnOnce(T)) -> ();
}

impl<T: Clone> CheckedSend<T> for Sender<T> {
    fn checked_send(&self, message: T, callback: impl FnOnce(T)) -> () {
        let error_message = message.clone();
        if self.send(message).is_err() { callback(error_message) }
    }
}