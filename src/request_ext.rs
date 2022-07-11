use crate::FlashMessage;

pub trait RequestFlashExt {
    fn flash(&self) -> Option<Vec<FlashMessage>>;
}

impl<State> RequestFlashExt for tide::Request<State> {
    fn flash(&self) -> Option<Vec<FlashMessage>> {
        match self.cookie("_flash") {
            None => None,
            Some(cookie) => {
                let messages: Vec<FlashMessage> =
                    serde_json::from_str(cookie.value()).unwrap_or_default();
                Some(messages)
            }
        }
    }
}
