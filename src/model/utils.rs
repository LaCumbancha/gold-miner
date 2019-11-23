use std::sync::mpsc::Sender;

pub trait CheckedSend {
    fn checked_send<T>(self, message: T, callback: &dyn Fn() -> ()) -> ();
}

impl CheckedSend for Sender<T> {
    fn checked_send(self, message: T, callback: &dyn Fn() -> ()) -> () {
        if self.send(message).is_err() { callback() }
    }
}