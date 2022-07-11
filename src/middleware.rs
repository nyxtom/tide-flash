use async_trait::async_trait;
use tide::{Middleware, Next, Request};

use crate::{Flash, FlashStore};

pub struct FlashMiddleware<Store> {
    pub store: Store,
}

impl<Store: FlashStore> FlashMiddleware<Store> {
    pub fn new(store: Store) -> Self {
        Self { store }
    }
}

#[async_trait]
impl<State, Store> Middleware<State> for FlashMiddleware<Store>
where
    State: Clone + Send + Sync + 'static,
    Store: FlashStore,
{
    async fn handle(&self, request: Request<State>, next: Next<'_, State>) -> tide::Result {
        let messages = self.store.load(&request);
        let mut res = next.run(request).await;
        if res.ext::<Flash>().is_some() {
            self.store.insert(&mut res);
        } else if messages.is_some() && !messages.unwrap().is_empty() {
            self.store.clear(&mut res);
        }
        Ok(res)
    }
}
