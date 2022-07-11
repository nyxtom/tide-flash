use crate::FlashMessage;

pub trait FlashStore: std::fmt::Debug + Send + Sync + 'static {
    fn load<State>(&self, req: &tide::Request<State>) -> Option<Vec<FlashMessage>>;
    fn insert(&self, res: &mut tide::Response);
    fn clear(&self, res: &mut tide::Response);
}
