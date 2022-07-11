use crate::{Flash, FlashMessage};

pub trait ResponseFlashExt {
    fn flash(&mut self, level: &str, msg: String);
    fn success<S: Into<String>>(&mut self, msg: S);
    fn info<S: Into<String>>(&mut self, msg: S);
    fn warn<S: Into<String>>(&mut self, msg: S);
    fn error<S: Into<String>>(&mut self, msg: S);
    fn debug<S: Into<String>>(&mut self, msg: S);
}

impl ResponseFlashExt for tide::Response {
    fn flash(&mut self, level: &str, message: String) {
        let mut messages = if let Some(flash) = self.ext::<Flash>() {
            flash.clone().messages
        } else {
            Vec::new()
        };
        messages.push(FlashMessage {
            level: String::from(level),
            message,
        });
        self.insert_ext::<Flash>(Flash { messages });
    }

    fn success<S: Into<String>>(&mut self, msg: S) {
        self.flash("success", msg.into());
    }

    fn info<S: Into<String>>(&mut self, msg: S) {
        self.flash("info", msg.into());
    }

    fn warn<S: Into<String>>(&mut self, msg: S) {
        self.flash("warn", msg.into());
    }

    fn error<S: Into<String>>(&mut self, msg: S) {
        self.flash("error", msg.into());
    }

    fn debug<S: Into<String>>(&mut self, msg: S) {
        self.flash("debug", msg.into());
    }
}
