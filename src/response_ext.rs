use crate::{Flash, FlashMessage};

pub trait ResponseFlashExt {
    fn flash(&mut self, level: &str, msg: String);
    fn flash_success<S: Into<String>>(&mut self, msg: S);
    fn flash_info<S: Into<String>>(&mut self, msg: S);
    fn flash_warn<S: Into<String>>(&mut self, msg: S);
    fn flash_error<S: Into<String>>(&mut self, msg: S);
    fn flash_debug<S: Into<String>>(&mut self, msg: S);
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
        println!("{:?}", messages);
        self.insert_ext::<Flash>(Flash { messages });
    }

    fn flash_success<S: Into<String>>(&mut self, msg: S) {
        self.flash("success", msg.into());
    }

    fn flash_info<S: Into<String>>(&mut self, msg: S) {
        self.flash("info", msg.into());
    }

    fn flash_warn<S: Into<String>>(&mut self, msg: S) {
        self.flash("warn", msg.into());
    }

    fn flash_error<S: Into<String>>(&mut self, msg: S) {
        self.flash("error", msg.into());
    }

    fn flash_debug<S: Into<String>>(&mut self, msg: S) {
        self.flash("debug", msg.into());
    }
}
