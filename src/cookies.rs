use http_types::cookies::SameSite;
use tide::http::cookies::Cookie;

use crate::{Flash, FlashMessage, FlashStore};

#[derive(Debug)]
pub struct CookieConfig {
    pub max_age: time::Duration,
    pub site: SameSite,
    pub http_only: bool,
    pub path: String,
}

impl Default for CookieConfig {
    fn default() -> Self {
        Self {
            max_age: time::Duration::seconds(60),
            site: SameSite::Lax,
            http_only: true,
            path: String::from("/"),
        }
    }
}

#[derive(Debug)]
pub struct CookieStore {
    pub config: CookieConfig,
}

impl Default for CookieStore {
    fn default() -> Self {
        Self {
            config: CookieConfig::default(),
        }
    }
}

impl FlashStore for CookieStore {
    fn load<State>(&self, req: &tide::Request<State>) -> Option<Vec<FlashMessage>> {
        match req.cookie("_flash") {
            None => None,
            Some(cookie) => {
                let messages: Vec<FlashMessage> =
                    serde_json::from_str(cookie.value()).unwrap_or_default();
                Some(messages)
            }
        }
    }

    fn insert(&self, res: &mut tide::Response) {
        if let Some(flash) = res.ext::<Flash>() {
            res.insert_cookie(
                Cookie::build(
                    "_flash",
                    serde_json::to_string(&flash.messages).unwrap_or_default(),
                )
                .max_age(self.config.max_age)
                .same_site(self.config.site)
                .http_only(self.config.http_only)
                .path(self.config.path.clone())
                .finish(),
            );
        }
    }

    fn clear(&self, res: &mut tide::Response) {
        res.insert_cookie(
            Cookie::build("_flash", "")
                .max_age(time::Duration::seconds(0))
                .same_site(self.config.site)
                .http_only(self.config.http_only)
                .path(self.config.path.clone())
                .finish(),
        );
    }
}
